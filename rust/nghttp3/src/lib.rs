//! Safe Rust bindings for nghttp3.
//!
//! The [`sys`] module exposes the complete generated C API. This crate adds
//! ownership, callback, error, header, settings, and connection abstractions for
//! the common HTTP/3 integration path.

use std::collections::HashMap;
use std::ffi::{c_void, CStr};
use std::fmt;
use std::marker::PhantomData;
use std::ptr::{self, NonNull};
use std::rc::Rc;
use std::slice;

pub use nghttp3_sys as sys;

const MAX_VARINT: u64 = (1_u64 << 62) - 1;

include!("part1.rs");
include!("part2.rs");
include!("part3.rs");
include!("part4.rs");
