/// Define an enumeration for version comparison
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum VersionComparison {
    Greater,
    Less,
    Equal,
}
