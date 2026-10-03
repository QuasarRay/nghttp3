//! HTTP/3 control-stream state machine.
//!
//! Initial rules are derived from RFC 9114 sections 6.2.1 and 7.2.4:
//! SETTINGS is the first control-stream frame and cannot subsequently repeat.

use lambars::optics::Lens;
use lambars::{lens, pipe};

use crate::generated::iana;
use crate::provenance::RuleSource;

/// RFC 9114 section 6.2.1: first control frame must be SETTINGS.
pub const SETTINGS_FIRST: RuleSource =
    RuleSource::new(9114, "6.2.1", "control.settings.first");

/// RFC 9114 section 7.2.4: SETTINGS must not occur subsequently.
pub const SETTINGS_ONCE: RuleSource =
    RuleSource::new(9114, "7.2.4", "control.settings.once");

/// HTTP/3 frames relevant to the initial control-stream state model.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Frame {
    /// SETTINGS (type 0x04).
    Settings,
    /// GOAWAY (type 0x07).
    Goaway,
    /// CANCEL_PUSH (type 0x03).
    CancelPush,
    /// MAX_PUSH_ID (type 0x0d).
    MaxPushId,
    /// Any extension/reserved frame not otherwise modeled here.
    Other(u64),
}

/// Connection errors produced by the modeled control-stream rules.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u64)]
pub enum ConnectionError {
    /// H3_FRAME_UNEXPECTED.
    FrameUnexpected = iana::error::H3_FRAME_UNEXPECTED,
    /// H3_MISSING_SETTINGS.
    MissingSettings = iana::error::H3_MISSING_SETTINGS,
}

/// Pure immutable state for one peer's HTTP/3 control stream.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ControlState {
    /// Whether the mandatory initial SETTINGS frame has been accepted.
    pub settings_seen: bool,
}

/// Successful or failed application of one protocol event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Transition {
    /// Event was accepted and produced the returned next state.
    Accepted(ControlState),
    /// Event violates a normative rule.
    ConnectionError {
        /// HTTP/3 application error code.
        error: ConnectionError,
        /// Exact normative rule responsible for the result.
        source: RuleSource,
    },
}

type Step = Result<(ControlState, Frame), (ConnectionError, RuleSource)>;

fn require_initial_settings((state, frame): (ControlState, Frame)) -> Step {
    if !state.settings_seen && frame != Frame::Settings {
        Err((ConnectionError::MissingSettings, SETTINGS_FIRST))
    } else {
        Ok((state, frame))
    }
}

fn reject_duplicate_settings((state, frame): (ControlState, Frame)) -> Step {
    if state.settings_seen && frame == Frame::Settings {
        Err((ConnectionError::FrameUnexpected, SETTINGS_ONCE))
    } else {
        Ok((state, frame))
    }
}

fn apply_frame((state, frame): (ControlState, Frame)) -> (ControlState, Frame) {
    let next = if frame == Frame::Settings {
        lens!(ControlState, settings_seen).set(state, true)
    } else {
        state
    };
    (next, frame)
}

fn finish(result: Step) -> Transition {
    match result {
        Ok((state, _)) => Transition::Accepted(state),
        Err((error, source)) => Transition::ConnectionError { error, source },
    }
}

/// Applies one frame to a peer control-stream state.
///
/// The Lambars pipeline makes the specification order explicit:
/// required-first-frame check -> duplicate check -> immutable state update.
pub fn receive(state: ControlState, frame: Frame) -> Transition {
    pipe!(
        Ok((state, frame)),
        =>> require_initial_settings,
        =>> reject_duplicate_settings,
        => apply_frame,
        finish
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_settings_is_required() {
        assert_eq!(
            receive(ControlState::default(), Frame::Goaway),
            Transition::ConnectionError {
                error: ConnectionError::MissingSettings,
                source: SETTINGS_FIRST,
            }
        );
    }

    #[test]
    fn initial_settings_advances_immutable_state() {
        assert_eq!(
            receive(ControlState::default(), Frame::Settings),
            Transition::Accepted(ControlState {
                settings_seen: true,
            })
        );
    }

    #[test]
    fn duplicate_settings_is_rejected() {
        let state = ControlState {
            settings_seen: true,
        };
        assert_eq!(
            receive(state, Frame::Settings),
            Transition::ConnectionError {
                error: ConnectionError::FrameUnexpected,
                source: SETTINGS_ONCE,
            }
        );
    }

    #[test]
    fn later_control_frame_preserves_state() {
        let state = ControlState {
            settings_seen: true,
        };
        assert_eq!(receive(state, Frame::Goaway), Transition::Accepted(state));
    }
}
