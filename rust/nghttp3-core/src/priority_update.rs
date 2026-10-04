//! Owned PRIORITY_UPDATE construction and queue-transfer semantics.
//!
//! The historical C implementation allocated a payload buffer before reserving
//! a frame-queue entry and had to remember to free that buffer on reservation
//! failure. Rust keeps the payload in an owned pending value and transfers it
//! only after reservation succeeds.

/// A PRIORITY_UPDATE frame whose payload is still owned by the caller.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PendingPriorityUpdate {
    stream_id: i64,
    data: Vec<u8>,
}

impl PendingPriorityUpdate {
    pub fn new(stream_id: i64, data: impl Into<Vec<u8>>) -> Self {
        Self {
            stream_id,
            data: data.into(),
        }
    }

    pub const fn stream_id(&self) -> i64 {
        self.stream_id
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Transfers this update into queued ownership only after `reserve`
    /// succeeds. On failure, the complete pending value is returned to the
    /// caller, so ordinary Rust drop semantics retain a single clear owner.
    pub fn try_queue<E>(
        self,
        reserve: impl FnOnce() -> Result<(), E>,
    ) -> Result<QueuedPriorityUpdate, EnqueueError<E>> {
        match reserve() {
            Ok(()) => Ok(QueuedPriorityUpdate {
                stream_id: self.stream_id,
                data: self.data,
            }),
            Err(error) => Err(EnqueueError {
                error,
                pending: self,
            }),
        }
    }
}

/// A PRIORITY_UPDATE frame whose payload ownership has transferred to the queue.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueuedPriorityUpdate {
    stream_id: i64,
    data: Vec<u8>,
}

impl QueuedPriorityUpdate {
    pub const fn stream_id(&self) -> i64 {
        self.stream_id
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

/// Reservation failure carrying the still-owned pending frame.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnqueueError<E> {
    pub error: E,
    pub pending: PendingPriorityUpdate,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_reservation_returns_payload_ownership_regression_9bf7d876() {
        let pending = PendingPriorityUpdate::new(4, b"u=1".to_vec());
        let error = pending.try_queue(|| Err::<(), _>("oom")).unwrap_err();

        assert_eq!(error.error, "oom");
        assert_eq!(error.pending.stream_id(), 4);
        assert_eq!(error.pending.data(), b"u=1");
    }

    #[test]
    fn successful_reservation_transfers_payload_once() {
        let pending = PendingPriorityUpdate::new(8, b"i".to_vec());
        let queued = pending.try_queue(|| Ok::<(), ()>(())).unwrap();

        assert_eq!(queued.stream_id(), 8);
        assert_eq!(queued.data(), b"i");
    }

    #[test]
    fn empty_payload_is_owned_without_special_case() {
        let pending = PendingPriorityUpdate::new(12, Vec::new());
        let queued = pending.try_queue(|| Ok::<(), ()>(())).unwrap();
        assert!(queued.data().is_empty());
    }
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn reservation_failure_preserves_exact_pending_ownership() {
        let stream_id: i64 = kani::any();
        let byte: u8 = kani::any();
        let pending = PendingPriorityUpdate::new(stream_id, vec![byte]);

        let error = pending.try_queue(|| Err::<(), _>(())).unwrap_err();

        assert_eq!(error.pending.stream_id(), stream_id);
        assert_eq!(error.pending.data().len(), 1);
        assert_eq!(error.pending.data()[0], byte);
    }

    #[kani::proof]
    fn reservation_success_preserves_exact_queued_payload() {
        let stream_id: i64 = kani::any();
        let byte: u8 = kani::any();
        let pending = PendingPriorityUpdate::new(stream_id, vec![byte]);

        let queued = pending.try_queue(|| Ok::<(), ()>(())).unwrap();

        assert_eq!(queued.stream_id(), stream_id);
        assert_eq!(queued.data().len(), 1);
        assert_eq!(queued.data()[0], byte);
    }
}
