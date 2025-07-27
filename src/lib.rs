//! compare_version
//!
//! A Rust library for comparing semantic versioning
//! strings and checking version compatibility.

pub(crate) mod cfg;
pub(crate) mod compare_version;

pub use compare_version::*;

pub(crate) use std::fmt;
