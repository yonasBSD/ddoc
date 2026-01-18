pub const DDOC_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn is_current_version_older_than(version: &str) -> bool {
    compare_versions(DDOC_VERSION, version) == std::cmp::Ordering::Less
}

pub fn compare_versions(
    a: &str,
    b: &str,
) -> std::cmp::Ordering {
    let a_parts: Vec<u32> = a.split('.').map(|s| s.parse().unwrap_or(0)).collect();
    let b_parts: Vec<u32> = b.split('.').map(|s| s.parse().unwrap_or(0)).collect();

    for (a_part, b_part) in a_parts.iter().zip(b_parts.iter()) {
        match a_part.cmp(b_part) {
            std::cmp::Ordering::Equal => continue,
            non_eq => return non_eq,
        }
    }

    a_parts.len().cmp(&b_parts.len())
}

#[test]
fn test_compare_versions() {
    assert_eq!(
        compare_versions("1.0.0", "1.0.0"),
        std::cmp::Ordering::Equal
    );
    assert_eq!(
        compare_versions("1.0.1", "1.0.0"),
        std::cmp::Ordering::Greater
    );
    assert_eq!(compare_versions("1.0.0", "1.0.1"), std::cmp::Ordering::Less);
    assert_eq!(compare_versions("1.1", "2"), std::cmp::Ordering::Less);
    assert_eq!(
        compare_versions(DDOC_VERSION, DDOC_VERSION),
        std::cmp::Ordering::Equal
    );
    assert!(!is_current_version_older_than(DDOC_VERSION));
}
