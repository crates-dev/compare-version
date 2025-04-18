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
