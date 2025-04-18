pub(crate) mod cfg;
pub(crate) mod r#enum;
pub(crate) mod error;
pub(crate) mod r#fn;
pub(crate) mod r#impl;
pub(crate) mod r#struct;

/// Version comparison result
pub use r#enum::*;
/// Version error information
pub use error::*;
/// Comparison functions
pub use r#fn::{compare_versions, matches_version_range};

pub(crate) use std::fmt;
pub(crate) use r#struct::*;
