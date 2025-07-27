/// Result of comparing two versions.
///
/// Indicates whether the first version is greater than, less than, or equal to the second version.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum VersionComparison {
    /// Indicates that the first version is greater than the second version.
    Greater,
    /// Indicates that the first version is less than the second version.
    Less,
    /// Indicates that the two versions are equal.
    Equal,
}
