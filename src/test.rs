#[test]
fn read_cur() {
    use super::*;

    let files = DirIterator::new()
        .expect("path not found")
        .flatten()
        .map(|e| e.file_name().as_os_str().to_string_lossy().to_string())
        .collect::<Vec<_>>();

    assert!(files.contains(&"test.rs".to_string()));
    assert!(files.contains(&"lib.rs".to_string()));
    assert!(files.contains(&"Cargo.toml".to_string()));
    assert!(files.contains(&"README.md".to_string()));
}

#[test]
fn read_dir() {
    use super::*;

    assert_eq!(
        DirIterator::from_path("src")
            .expect("path not found")
            .flatten()
            .map(|e| e.file_name().as_os_str().to_string_lossy().to_string())
            .collect::<Vec<_>>(),
        ["lib.rs", "test.rs"],
    );
}

#[cfg(feature = "wildcard")]
#[test]
fn filter_dir() {
    use super::*;

    assert_eq!(
        DirIterator::from_path("src")
            .expect("path not found")
            .flatten()
            .filter(wildcard("test.*"))
            .map(|e| e.file_name().as_os_str().to_string_lossy().to_string())
            .collect::<Vec<_>>(),
        ["test.rs"],
    );
}
