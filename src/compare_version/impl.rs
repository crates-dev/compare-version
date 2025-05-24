use crate::*;

impl Version {
    /// Parses a version from a string.
    ///
    /// # Arguments
    ///
    /// - `version` - A string slice that holds the version in the format 'x.y.z'.
    ///
    /// # Errors
    ///
    /// Returns a `VersionError` if the version format is invalid or if any version
    /// components cannot be parsed.
    pub(crate) fn parse(version: &str) -> Result<Self, VersionError> {
        let mut parts: Vec<&str> = version.split('.').collect();
        let (patch_part, pre_release) = if let Some(patch_with_prerelease) = parts.pop() {
            let mut patch_parts = patch_with_prerelease.splitn(2, '-');
            (
                patch_parts.next().unwrap_or(""),
                patch_parts.next().map(|s| s.to_string()),
            )
        } else {
            return Err(VersionError::ParseError(
                "Version format error, should be in the form 'x.y.z'.".to_string(),
            ));
        };
        let major: u32 = parts
            .get(0)
            .unwrap_or(&"0")
            .parse::<u32>()
            .map_err(|_| VersionError::MajorVersionError)?;
        let minor: u32 = parts
            .get(1)
            .unwrap_or(&"0")
            .parse::<u32>()
            .map_err(|_| VersionError::MinorVersionError)?;
        let patch: u32 = patch_part
            .parse::<u32>()
            .map_err(|_| VersionError::PatchVersionError)?;
        Ok(Self {
            major,
            minor,
            patch,
            pre_release,
        })
    }
}

impl CompareVersion {
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
    /// let result = CompareVersion::compare_version("1.2.3", "1.2.4");
    /// assert_eq!(result, Ok(VersionComparison::Less));
    /// ```
    pub fn compare_version(
        version1: &str,
        version2: &str,
    ) -> Result<VersionComparison, VersionError> {
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
    /// let matches = CompareVersion::matches_version_range("1.2.3", "^1.2.0");
    /// assert_eq!(matches, Ok(true));
    /// ```
    ///
    /// ```
    /// use compare_version::*;
    /// let matches = CompareVersion::matches_version_range("1.2.3", "~1.2.4");
    /// assert_eq!(matches, Ok(false));
    /// ```
    pub fn matches_version_range(version: &str, range: &str) -> Result<bool, VersionError> {
        let target_version: Version = Version::parse(version)?;
        if let Some(stripped_range) = range.strip_prefix('^') {
            let base_version = Version::parse(stripped_range)?;
            // `^` indicates major version compatibility
            Ok(target_version.major == base_version.major
                && (target_version.minor > base_version.minor
                    || (target_version.minor == base_version.minor
                        && target_version >= base_version)))
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
}
