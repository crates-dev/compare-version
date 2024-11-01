use std::fmt;

/// Define an enumeration for version errors
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum VersionError {
    ParseError(String),
    InvalidRangeFormat,
    MajorVersionError,
    MinorVersionError,
    PatchVersionError,
}

impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VersionError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            VersionError::InvalidRangeFormat => {
                write!(f, "Unsupported range format, only '^' or '~' are supported")
            }
            VersionError::MajorVersionError => write!(f, "Major version parsing error"),
            VersionError::MinorVersionError => write!(f, "Minor version parsing error"),
            VersionError::PatchVersionError => write!(f, "Patch version parsing error"),
        }
    }
}
