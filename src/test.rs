#[test]
fn read_dir() {
    use super::*;

    assert_eq!(
        DirIterator::new("src")
            .expect("path not found")
            .flatten()
            .map(|e| e.file_name().as_os_str().to_string_lossy().to_string())
            .collect::<Vec<_>>(),
        ["lib.rs", "test.rs"],
    );
}

#[test]
fn filter_dir() {
    use super::*;

    assert_eq!(
        DirIterator::new("src")
            .expect("path not found")
            .flatten()
            .filter(wildcard("test.*"))
            .map(|e| e.file_name().as_os_str().to_string_lossy().to_string())
            .collect::<Vec<_>>(),
        ["test.rs"],
    );
}
