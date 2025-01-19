use super::{
    error::VersionError,
    r#type::{Version, VersionComparison},
};

/// Compares two version strings and returns a `VersionComparison` enum.
///
/// # Arguments
///
/// - `version1` - A string slice that holds the first version to compare.
/// - `version2` - A string slice that holds the second version to compare.
///
/// # Errors
///
/// Returns a `VersionError` if either version cannot be parsed.
///
/// # Examples
///
/// ```
/// use compare_version::*;
/// let result = compare_versions("1.2.3", "1.2.4");
/// assert_eq!(result, Ok(VersionComparison::Less));
/// ```
#[inline]
pub fn compare_versions(version1: &str, version2: &str) -> Result<VersionComparison, VersionError> {
    let v1: Version = Version::parse(version1)?;
    let v2: Version = Version::parse(version2)?;
    match v1.cmp(&v2) {
        std::cmp::Ordering::Greater => Ok(VersionComparison::Greater),
        std::cmp::Ordering::Less => Ok(VersionComparison::Less),
        std::cmp::Ordering::Equal => Ok(VersionComparison::Equal),
    }
}

/// Checks whether a version matches a specified range, supporting `^` and `~` ranges.
///
/// # Arguments
///
/// - `version` - A string slice that holds the version to check against the range.
/// - `range` - A string slice that holds the version range to check.
///
/// # Errors
///
/// Returns a `VersionError` if the version or range cannot be parsed.
///
/// # Examples
///
/// ```
/// use compare_version::*;
/// let matches = matches_version_range("1.2.3", "^1.2.0");
/// assert_eq!(matches, Ok(true));
/// ```
///
/// ```
/// use compare_version::*;
/// let matches = matches_version_range("1.2.3", "~1.2.4");
/// assert_eq!(matches, Ok(false));
/// ```
#[inline]
pub fn matches_version_range(version: &str, range: &str) -> Result<bool, VersionError> {
    let target_version: Version = Version::parse(version)?;
    if let Some(stripped_range) = range.strip_prefix('^') {
        let base_version = Version::parse(stripped_range)?;
        // `^` indicates major version compatibility
        Ok(target_version.major == base_version.major
            && (target_version.minor > base_version.minor
                || (target_version.minor == base_version.minor && target_version >= base_version)))
    } else if let Some(stripped_range) = range.strip_prefix('~') {
        let base_version: Version = Version::parse(stripped_range)?;
        // `~` indicates minor version compatibility
        Ok(target_version.major == base_version.major
            && target_version.minor == base_version.minor
            && target_version >= base_version)
    } else {
        Err(VersionError::InvalidRangeFormat)
    }
}
