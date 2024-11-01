mod cfg;
mod compare;
mod error;
mod r#impl;
mod r#type;

/// Comparison functions
pub use compare::{compare_versions, matches_version_range};
/// Version error information
pub use error::VersionError;
/// Version comparison result
pub use r#type::VersionComparison;
