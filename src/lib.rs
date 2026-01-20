//! compare_version
//!
//! A Rust library for comparing semantic versioning
//! strings and checking version compatibility.

pub(crate) mod r#enum;
pub(crate) mod r#impl;
pub(crate) mod r#struct;

#[cfg(test)]
mod test;

pub use {r#enum::*, r#struct::*};

pub(crate) use std::fmt;
