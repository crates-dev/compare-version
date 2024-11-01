#[test]
fn test_compare_versions() {
    use crate::*;
    let version1: &str = "1.2.3-alpha";
    let version2: &str = "1.2.3-alpha";
    let result: Result<VersionComparison, VersionError> = compare_versions(version1, version2);
    assert_eq!(result.unwrap(), VersionComparison::Equal);
    let version1: &str = "1.2.3";
    let version2: &str = "1.2.3-alpha";
    let result: Result<VersionComparison, VersionError> = compare_versions(version1, version2);
    assert_eq!(result.unwrap(), VersionComparison::Less);
    let version1: &str = "1.2.2";
    let version2: &str = "1.2.3-alpha";
    let result: Result<VersionComparison, VersionError> = compare_versions(version1, version2);
    assert_eq!(result.unwrap(), VersionComparison::Less);
    let version1: &str = "1.2.4";
    let version2: &str = "1.2.3-alpha";
    let result: Result<VersionComparison, VersionError> = compare_versions(version1, version2);
    assert_eq!(result.unwrap(), VersionComparison::Greater);
    let version1: &str = "1.2.4";
    let version2: &str = "1.2.3";
    let result: Result<VersionComparison, VersionError> = compare_versions(version1, version2);
    assert_eq!(result.unwrap(), VersionComparison::Greater);
    let version1: &str = "1.2.4";
    let version2: &str = "1.2.5";
    let result: Result<VersionComparison, VersionError> = compare_versions(version1, version2);
    assert_eq!(result.unwrap(), VersionComparison::Less);
    let version1: &str = "1.2.4";
    let version2: &str = "1.2.4";
    let result: Result<VersionComparison, VersionError> = compare_versions(version1, version2);
    assert_eq!(result.unwrap(), VersionComparison::Equal);
}

#[test]
fn test_matches_version_range() {
    use crate::*;
    let version: &str = "1.2.3-alpha";
    let range1: &str = "^1.2.0";
    let range2: &str = "~1.2.3";
    let result: bool = match matches_version_range(version, range1) {
        Ok(matches) => matches,
        Err(_) => false,
    };
    assert_eq!(result, true);
    let result: bool = match matches_version_range(version, range2) {
        Ok(matches) => matches,
        Err(_) => false,
    };
    assert_eq!(result, true);

    let version: &str = "1.2.5-alpha";
    let range1: &str = "^1.2.0";
    let range2: &str = "~1.2.3";
    let result: bool = match matches_version_range(version, range1) {
        Ok(matches) => matches,
        Err(_) => false,
    };
    assert_eq!(result, true);
    let result: bool = match matches_version_range(version, range2) {
        Ok(matches) => matches,
        Err(_) => false,
    };
    assert_eq!(result, true);

    let version: &str = "1.3.0";
    let range1: &str = "^1.2.0";
    let range2: &str = "~1.2.3";
    let result: bool = match matches_version_range(version, range1) {
        Ok(matches) => matches,
        Err(_) => false,
    };
    assert_eq!(result, true);
    let result: bool = match matches_version_range(version, range2) {
        Ok(matches) => matches,
        Err(_) => false,
    };
    assert_eq!(result, false);

    let version: &str = "2.0.0";
    let range1: &str = "^1.2.0";
    let range2: &str = "~1.2.3";
    let result: bool = match matches_version_range(version, range1) {
        Ok(matches) => matches,
        Err(_) => false,
    };
    assert_eq!(result, false);
    let result: bool = match matches_version_range(version, range2) {
        Ok(matches) => matches,
        Err(_) => false,
    };
    assert_eq!(result, false);

    let version: &str = "1.0.0";
    let range1: &str = "^1.2.0";
    let range2: &str = "~1.2.3";
    let result: bool = match matches_version_range(version, range1) {
        Ok(matches) => matches,
        Err(_) => false,
    };
    assert_eq!(result, false);
    let result: bool = match matches_version_range(version, range2) {
        Ok(matches) => matches,
        Err(_) => false,
    };
    assert_eq!(result, false);
}
