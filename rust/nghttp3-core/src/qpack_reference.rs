//! Transactional ownership for QPACK stream references.
//!
//! Historical commit 62743057 fixed a double free caused by publishing a
//! heap-owned reference into the stream ring buffer *before* a fallible
//! priority-queue registration. If registration failed, the caller freed the
//! reference while the ring buffer still retained the pointer.
//!
//! Rust typestate makes the required order structural:
//!
//! PendingReference<T> --fallible registration--> RegisteredReference<T, R>
//! RegisteredReference<T, R> --infallible publish--> StreamReferences<T, R>
//!
//! A stream cannot publish a PendingReference.

use std::collections::VecDeque;

/// A reference value that has not yet completed external registration.
#[derive(Debug, Eq, PartialEq)]
#[must_use = "pending QPACK references must be registered or deliberately dropped"]
pub struct PendingReference<T> {
    value: T,
}

impl<T> PendingReference<T> {
    /// Starts a new unpublished reference.
    pub const fn new(value: T) -> Self {
        Self { value }
    }

    /// Borrows the unpublished value.
    pub const fn value(&self) -> &T {
        &self.value
    }

    /// Performs the fallible registration step.
    ///
    /// Success returns a distinct type accepted by StreamReferences::publish.
    /// Failure returns ownership to the caller; nothing has been published.
    pub fn register<R, E, F>(
        self,
        register: F,
    ) -> Result<RegisteredReference<T, R>, RegistrationFailure<T, E>>
    where
        F: FnOnce(&T) -> Result<R, E>,
    {
        match register(&self.value) {
            Ok(registration) => Ok(RegisteredReference {
                value: self.value,
                registration,
            }),
            Err(error) => Err(RegistrationFailure {
                pending: self,
                error,
            }),
        }
    }

    /// Deliberately recovers the unpublished value.
    pub fn into_inner(self) -> T {
        self.value
    }
}

/// A failed registration that still owns the unpublished reference.
#[derive(Debug, Eq, PartialEq)]
pub struct RegistrationFailure<T, E> {
    pending: PendingReference<T>,
    error: E,
}

impl<T, E> RegistrationFailure<T, E> {
    /// Borrows the registration error.
    pub const fn error(&self) -> &E {
        &self.error
    }

    /// Recovers both the pending value and the registration error.
    pub fn into_parts(self) -> (PendingReference<T>, E) {
        (self.pending, self.error)
    }
}

/// A reference whose external registration has succeeded.
#[derive(Debug, Eq, PartialEq)]
#[must_use = "registered QPACK references must be published or deliberately dropped"]
pub struct RegisteredReference<T, R> {
    value: T,
    registration: R,
}

impl<T, R> RegisteredReference<T, R> {
    /// Borrows the registered value.
    pub const fn value(&self) -> &T {
        &self.value
    }

    /// Borrows the registration token/guard.
    pub const fn registration(&self) -> &R {
        &self.registration
    }

    /// Recovers both owned components.
    pub fn into_parts(self) -> (T, R) {
        (self.value, self.registration)
    }
}

/// Published stream references.
///
/// The registration token is owned alongside the value. An RAII registration
/// guard can therefore unregister automatically when the published reference is
/// removed.
#[derive(Debug, Default)]
pub struct StreamReferences<T, R> {
    refs: VecDeque<RegisteredReference<T, R>>,
}

impl<T, R> StreamReferences<T, R> {
    /// Creates an empty publication set.
    pub const fn new() -> Self {
        Self {
            refs: VecDeque::new(),
        }
    }

    /// Returns the number of published references.
    pub fn len(&self) -> usize {
        self.refs.len()
    }

    /// Returns whether no reference has been published.
    pub fn is_empty(&self) -> bool {
        self.refs.is_empty()
    }

    /// Publishes a reference only after registration has succeeded.
    pub fn publish(&mut self, registered: RegisteredReference<T, R>) {
        self.refs.push_back(registered);
    }

    /// Borrows the oldest published reference.
    pub fn front(&self) -> Option<&RegisteredReference<T, R>> {
        self.refs.front()
    }

    /// Removes the oldest published reference and its registration guard.
    pub fn pop_front(&mut self) -> Option<RegisteredReference<T, R>> {
        self.refs.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn historical_registration_failure_never_publishes_62743057() {
        let pending = PendingReference::new(7_u8);
        let mut stream = StreamReferences::<u8, ()>::new();

        let failure = pending.register(|_| Err::<(), _>("oom")).unwrap_err();
        assert!(stream.is_empty());
        assert_eq!(failure.error(), &"oom");

        // The caller still owns exactly one unpublished value.
        let (pending, error) = failure.into_parts();
        assert_eq!(error, "oom");
        assert_eq!(pending.into_inner(), 7);
        assert!(stream.is_empty());
    }

    #[test]
    fn successful_registration_can_then_be_published() {
        let pending = PendingReference::new(7_u8);
        let registered = pending.register(|value| Ok::<_, ()>(*value + 1)).unwrap();

        let mut stream = StreamReferences::new();
        stream.publish(registered);

        assert_eq!(stream.len(), 1);
        assert_eq!(stream.front().unwrap().value(), &7);
        assert_eq!(stream.front().unwrap().registration(), &8);
    }

    #[test]
    fn removal_moves_value_and_registration_together() {
        let registered = PendingReference::new(11_u8)
            .register(|_| Ok::<_, ()>(22_u8))
            .unwrap();
        let mut stream = StreamReferences::new();
        stream.publish(registered);

        let published = stream.pop_front().unwrap();
        assert!(stream.is_empty());
        assert_eq!(published.into_parts(), (11, 22));
    }
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn failed_registration_keeps_stream_empty() {
        let value: u8 = kani::any();
        let error: u8 = kani::any();
        let pending = PendingReference::new(value);
        let mut stream = StreamReferences::<u8, ()>::new();

        let failure = pending
            .register(|_| Err::<(), _>(error))
            .expect_err("registration is modeled as failing");

        assert!(stream.is_empty());
        let (pending, observed_error) = failure.into_parts();
        assert_eq!(pending.into_inner(), value);
        assert_eq!(observed_error, error);
        assert!(stream.is_empty());
    }

    #[kani::proof]
    fn successful_registration_publishes_once() {
        let value: u8 = kani::any();
        let token: u8 = kani::any();
        let registered = PendingReference::new(value)
            .register(|_| Ok::<_, ()>(token))
            .unwrap();

        let mut stream = StreamReferences::new();
        stream.publish(registered);

        assert_eq!(stream.len(), 1);
        assert_eq!(stream.front().unwrap().value(), &value);
        assert_eq!(stream.front().unwrap().registration(), &token);
    }

    #[kani::proof]
    fn pop_moves_both_owners_out() {
        let value: u8 = kani::any();
        let token: u8 = kani::any();
        let registered = PendingReference::new(value)
            .register(|_| Ok::<_, ()>(token))
            .unwrap();

        let mut stream = StreamReferences::new();
        stream.publish(registered);
        let published = stream.pop_front().unwrap();

        assert!(stream.is_empty());
        assert_eq!(published.into_parts(), (value, token));
    }
}
