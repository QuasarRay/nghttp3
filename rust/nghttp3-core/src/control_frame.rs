//! Owned control-frame staging for fallible queue admission.
//!
//! Historical commit 9bf7d876 fixed a leak in
//! nghttp3_conn_set_client_stream_priority: a heap buffer was allocated and
//! filled before a fallible control-stream queue insertion, but the failure
//! branch returned without freeing that buffer.
//!
//! This module expresses the same lifecycle with ordinary Rust ownership. A
//! PendingControlFrame owns its bytes until admission succeeds. Failed admission
//! returns an AdmissionFailure that still owns the frame; returning/dropping that
//! error therefore releases the bytes automatically.

use std::collections::VecDeque;

/// An encoded control frame that has not yet entered the send queue.
#[derive(Debug, Eq, PartialEq)]
#[must_use = "a pending control frame must be enqueued, retried, or deliberately dropped"]
pub struct PendingControlFrame {
    bytes: Vec<u8>,
}

impl PendingControlFrame {
    /// Creates one pending owned frame.
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            bytes: bytes.into(),
        }
    }

    /// Borrows the encoded bytes.
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    /// Recovers the owned encoding.
    pub fn into_inner(self) -> Vec<u8> {
        self.bytes
    }

    /// Runs a fallible admission check and publishes only on success.
    pub fn admit<E, F>(
        self,
        queue: &mut ControlFrameQueue,
        admit: F,
    ) -> Result<(), AdmissionFailure<E>>
    where
        F: FnOnce(&[u8]) -> Result<(), E>,
    {
        match admit(&self.bytes) {
            Ok(()) => {
                queue.frames.push_back(self.bytes);
                Ok(())
            }
            Err(error) => Err(AdmissionFailure { frame: self, error }),
        }
    }
}

/// Failed admission retaining the sole unpublished frame owner.
#[derive(Debug, Eq, PartialEq)]
pub struct AdmissionFailure<E> {
    frame: PendingControlFrame,
    error: E,
}

impl<E> AdmissionFailure<E> {
    /// Borrows the admission error.
    pub const fn error(&self) -> &E {
        &self.error
    }

    /// Borrows the still-unpublished frame.
    pub const fn frame(&self) -> &PendingControlFrame {
        &self.frame
    }

    /// Recovers the frame for a retry together with its error.
    pub fn into_parts(self) -> (PendingControlFrame, E) {
        (self.frame, self.error)
    }
}

/// FIFO of owned control-frame encodings.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct ControlFrameQueue {
    frames: VecDeque<Vec<u8>>,
}

impl ControlFrameQueue {
    /// Creates an empty queue.
    pub const fn new() -> Self {
        Self {
            frames: VecDeque::new(),
        }
    }

    /// Number of admitted control frames.
    pub fn len(&self) -> usize {
        self.frames.len()
    }

    /// Whether no frame has been admitted.
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    /// Borrows the oldest encoded frame.
    pub fn front(&self) -> Option<&[u8]> {
        self.frames.front().map(Vec::as_slice)
    }

    /// Removes the oldest frame, transferring byte ownership to the caller.
    pub fn pop_front(&mut self) -> Option<Vec<u8>> {
        self.frames.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn historical_failure_path_retains_no_queued_owner_9bf7d876() {
        let frame = PendingControlFrame::new(b"priority-update".to_vec());
        let mut queue = ControlFrameQueue::new();

        let failure = frame.admit(&mut queue, |_| Err("oom")).unwrap_err();

        assert!(queue.is_empty());
        assert_eq!(failure.error(), &"oom");
        assert_eq!(failure.frame().as_slice(), b"priority-update");

        // Returning this error from the caller would drop the sole owner.
        drop(failure);
        assert!(queue.is_empty());
    }

    #[test]
    fn successful_admission_moves_bytes_into_queue() {
        let frame = PendingControlFrame::new(b"priority-update".to_vec());
        let mut queue = ControlFrameQueue::new();

        frame.admit::<(), _>(&mut queue, |_| Ok(())).unwrap();

        assert_eq!(queue.len(), 1);
        assert_eq!(queue.front(), Some(&b"priority-update"[..]));
        assert_eq!(queue.pop_front(), Some(b"priority-update".to_vec()));
        assert!(queue.is_empty());
    }

    #[test]
    fn failed_frame_can_be_retried_without_copying() {
        let frame = PendingControlFrame::new(vec![1, 2, 3]);
        let mut queue = ControlFrameQueue::new();

        let failure = frame.admit(&mut queue, |_| Err(7_u8)).unwrap_err();
        let (frame, error) = failure.into_parts();
        assert_eq!(error, 7);
        assert_eq!(frame.as_slice(), &[1, 2, 3]);

        frame.admit::<(), _>(&mut queue, |_| Ok(())).unwrap();
        assert_eq!(queue.front(), Some(&[1, 2, 3][..]));
    }
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn failed_admission_never_publishes_symbolic_frame() {
        let byte: u8 = kani::any();
        let error: u8 = kani::any();
        let frame = PendingControlFrame::new(vec![byte]);
        let mut queue = ControlFrameQueue::new();

        let failure = frame
            .admit(&mut queue, |_| Err(error))
            .expect_err("admission is modeled as failing");

        assert!(queue.is_empty());
        assert_eq!(failure.frame().as_slice(), &[byte]);
        assert_eq!(failure.error(), &error);
    }

    #[kani::proof]
    fn successful_admission_publishes_exactly_once() {
        let byte: u8 = kani::any();
        let frame = PendingControlFrame::new(vec![byte]);
        let mut queue = ControlFrameQueue::new();

        frame.admit::<(), _>(&mut queue, |_| Ok(())).unwrap();

        assert_eq!(queue.len(), 1);
        assert_eq!(queue.front(), Some(&[byte][..]));
    }

    #[kani::proof]
    fn retry_preserves_symbolic_bytes() {
        let byte: u8 = kani::any();
        let frame = PendingControlFrame::new(vec![byte]);
        let mut queue = ControlFrameQueue::new();

        let failure = frame.admit(&mut queue, |_| Err(())).unwrap_err();
        let (frame, ()) = failure.into_parts();
        assert!(queue.is_empty());

        frame.admit::<(), _>(&mut queue, |_| Ok(())).unwrap();
        assert_eq!(queue.front(), Some(&[byte][..]));
    }
}
