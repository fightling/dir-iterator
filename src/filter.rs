//! Result Filters

use std::*;

/// We deal with std::io::Error
type Result<T> = std::io::Result<T>;

/// Inclusively filter by wildcards
#[cfg(feature = "wildcard")]
pub fn include(wildcard: &'static str) -> impl Fn(&fs::DirEntry) -> bool {
    let wildcard = wc::Wildcard::new(wildcard.as_bytes()).unwrap();
    move |entry| wildcard.is_match(entry.file_name().as_encoded_bytes())
}

/// Exclusively filter by wildcards
#[cfg(feature = "wildcard")]
pub fn exclude(wildcard: &'static str) -> impl Fn(&fs::DirEntry) -> bool {
    let wildcard = wc::Wildcard::new(wildcard.as_bytes()).unwrap();
    move |entry| !wildcard.is_match(entry.file_name().as_encoded_bytes())
}

/// filter files only
pub fn files(entry: &fs::DirEntry) -> bool {
    if let Ok(file_type) = entry.file_type() {
        file_type.is_file()
    } else {
        false
    }
}

/// filter directories only
pub fn dirs(entry: &fs::DirEntry) -> bool {
    if let Ok(file_type) = entry.file_type() {
        file_type.is_dir()
    } else {
        false
    }
}

/// filter symlinks only
pub fn symlink(entry: &fs::DirEntry) -> bool {
    if let Ok(file_type) = entry.file_type() {
        file_type.is_symlink()
    } else {
        false
    }
}
