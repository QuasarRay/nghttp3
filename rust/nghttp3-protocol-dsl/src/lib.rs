//! Declarative HTTP/3 semantic rules.
//!
//! This crate sits above the byte-level verified core. It intentionally uses
//! Lambars to compress pure state-machine rules while remaining independent of
//! any async executor or transport runtime.

#![forbid(unsafe_code)]

pub mod control;
pub mod generated;
pub mod provenance;
