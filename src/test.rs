#[test]
fn read_cur() {
    use super::*;

    let files = DirIterator::build_current()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect::<Vec<_>>();

    assert!(files.contains(&"test.rs".to_string()));
    assert!(files.contains(&"lib.rs".to_string()));
    assert!(files.contains(&"Cargo.toml".to_string()));
    assert!(files.contains(&"README.md".to_string()));
}

#[test]
fn read_dir() {
    use super::*;

    let mut dir = DirIterator::build_from_path("src")
        .expect("path not found")
        .map(|e| e.file_name().as_os_str().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    dir.sort();
    assert_eq!(dir, ["filter.rs", "lib.rs", "test.rs"],);
}

#[cfg(feature = "wildcard")]
#[test]
fn filter_dir() {
    use super::*;

    let mut dir = DirIterator::build_from_path("src")
        .expect("path not found")
        .filter(filter::include("test.*"))
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    dir.sort();
    assert_eq!(dir, ["test.rs"],);
}

#[test]
fn read_paths() {
    use super::*;

    let files = DirIterator::build_current()
        .map(|e| e.path())
        .collect::<Vec<_>>();

    let cur = env::current_dir().unwrap();
    assert!(files.contains(&cur.join("src/test.rs")));
    assert!(files.contains(&cur.join("src/lib.rs")));
    assert!(files.contains(&cur.join("Cargo.toml")));
    assert!(files.contains(&cur.join("README.md")));
}

#[cfg(feature = "wildcard")]
#[test]
fn filter_dirs() {
    use super::*;

    let dir = DirIterator::current()
        .expect("path not found")
        .ignore("target")
        .build()
        .filter(filter::dirs)
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect::<Vec<_>>();

    assert!(dir.contains(&".github".to_string()));
    assert!(!dir.contains(&"target".to_string()));
    assert!(!dir.contains(&"*.o".to_string()));
}

mod readme {
    use filter::exclude;

    use super::super::*;

    #[test]
    fn read_a_directory_recursively() {
        // build a new iterator starting in the current directory
        DirIterator::build_current()
            // print each file name
            .for_each(|e| println!("{:?}", e.file_name()));
    }

    #[test]
    fn filter_result_with_wildcards() {
        DirIterator::build_current()
            // filter all files which have extension `txt`
            .filter(exclude("*.txt"))
            .for_each(|e| println!("{:?}", e.file_name()));
    }

    #[test]
    fn ignore_folders_when_scanning() {
        DirIterator::current()
            .expect("path not found")
            // ignore target directory
            .ignore("target")
            // ignore all hidden directories
            .ignore(".*")
            // build iterator
            .build()
            // exclude all hidden files
            .filter(exclude(".*"))
            .for_each(|e| println!("{:?}", e.path()));
    }
}
