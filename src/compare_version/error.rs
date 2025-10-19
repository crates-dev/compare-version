use crate::*;

/// Errors that can occur during version parsing and comparison.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum VersionError {
    /// An error occurred during parsing, with a specific message.
    ParseError(String),
    /// The format of the version range is invalid.
    InvalidRangeFormat,
    /// An error occurred while parsing the major version component.
    MajorVersionError,
    /// An error occurred while parsing the minor version component.
    MinorVersionError,
    /// An error occurred while parsing the patch version component.
    PatchVersionError,
}

/// Implements the `Display` trait for `VersionError` to provide user-friendly error messages.
impl fmt::Display for VersionError {
    /// Formats the `VersionError` into a human-readable string.
    ///
    /// # Arguments
    ///
    /// - `&mut fmt::Formatter` - Formatter to write the output to.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Result of the formatting operation.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VersionError::ParseError(msg) => write!(f, "Parse error: {msg}"),
            VersionError::InvalidRangeFormat => {
                write!(f, "Unsupported range format, only '^' or '~' are supported")
            }
            VersionError::MajorVersionError => write!(f, "Major version parsing error"),
            VersionError::MinorVersionError => write!(f, "Minor version parsing error"),
            VersionError::PatchVersionError => write!(f, "Patch version parsing error"),
        }
    }
}
