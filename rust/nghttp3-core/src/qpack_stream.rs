//! Transactional publication ordering for QPACK stream references.
//!
//! Historical C code published a reference into the stream ring before adding
//! its priority-queue entry. If priority-queue insertion then failed, ownership
//! was ambiguous and cleanup could free the same reference twice. This API
//! registers first and publishes only after registration succeeds.

/// Stream-owned QPACK references that have completed prerequisite registration.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct PublishedRefs<T> {
    refs: Vec<T>,
}

impl<T> PublishedRefs<T> {
    pub const fn new() -> Self {
        Self { refs: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.refs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.refs.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.refs.get(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.refs.iter()
    }

    /// Registers `reference` with an external index before publishing it.
    ///
    /// If registration fails, this collection remains unchanged and the
    /// reference is dropped by the caller's normal Rust ownership path.
    pub fn register_then_publish<E>(
        &mut self,
        reference: T,
        register: impl FnOnce(&T) -> Result<(), E>,
    ) -> Result<(), E> {
        register(&reference)?;
        self.refs.push(reference);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        self.refs.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_registration_does_not_publish_regression_62743057() {
        let mut refs = PublishedRefs::new();
        let result = refs.register_then_publish(7_u8, |_| Err::<(), _>("oom"));

        assert_eq!(result, Err("oom"));
        assert!(refs.is_empty());
    }

    #[test]
    fn successful_registration_publishes_exactly_once() {
        let mut refs = PublishedRefs::new();
        assert_eq!(
            refs.register_then_publish(7_u8, |_| Ok::<(), ()>(())),
            Ok(())
        );
        assert_eq!(refs.len(), 1);
        assert_eq!(refs.get(0), Some(&7));
        assert_eq!(refs.pop(), Some(7));
        assert!(refs.is_empty());
    }

    #[test]
    fn failure_preserves_preexisting_publication_set() {
        let mut refs = PublishedRefs::new();
        refs.register_then_publish(1_u8, |_| Ok::<(), ()>(()))
            .unwrap();

        assert_eq!(
            refs.register_then_publish(2_u8, |_| Err::<(), _>("oom")),
            Err("oom")
        );
        assert_eq!(refs.iter().copied().collect::<Vec<_>>(), vec![1]);
    }
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn publication_occurs_iff_registration_succeeds() {
        let reference: u8 = kani::any();
        let fail: bool = kani::any();
        let mut refs = PublishedRefs::new();

        let result = refs.register_then_publish(reference, |_| {
            if fail {
                Err(())
            } else {
                Ok(())
            }
        });

        if fail {
            assert_eq!(result, Err(()));
            assert!(refs.is_empty());
        } else {
            assert_eq!(result, Ok(()));
            assert_eq!(refs.len(), 1);
            assert_eq!(refs.get(0), Some(&reference));
        }
    }

    #[kani::proof]
    fn failed_registration_preserves_existing_refs() {
        let first: u8 = kani::any();
        let second: u8 = kani::any();
        let mut refs = PublishedRefs::new();

        refs.register_then_publish(first, |_| Ok::<(), ()>(()))
            .unwrap();
        let result = refs.register_then_publish(second, |_| Err::<(), _>(()));

        assert_eq!(result, Err(()));
        assert_eq!(refs.len(), 1);
        assert_eq!(refs.get(0), Some(&first));
    }
}
