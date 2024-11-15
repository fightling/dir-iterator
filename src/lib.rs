//! Directory Iterator

#![allow(dead_code)]

#[cfg(test)]
mod test;

use std::*;

/// Used to filter flattened `DirIterator` with wildcards
#[cfg(feature = "wildcard")]
pub fn wildcard(wildcard: &'static str) -> impl FnMut(&fs::DirEntry) -> bool {
    let wildcard = wc::Wildcard::new(wildcard.as_bytes()).unwrap();
    move |entry| wildcard.is_match(entry.file_name().as_encoded_bytes())
}

#[cfg(feature = "wildcard")]
pub fn exclude(wildcard: &'static str) -> impl FnMut(&fs::DirEntry) -> bool {
    let wildcard = wc::Wildcard::new(wildcard.as_bytes()).unwrap();
    move |entry| !wildcard.is_match(entry.file_name().as_encoded_bytes())
}

pub fn files(entry: &fs::DirEntry) -> bool {
    if let Ok(file_type) = entry.file_type() {
        file_type.is_file()
    } else {
        false
    }
}

pub fn dirs(entry: &fs::DirEntry) -> bool {
    if let Ok(file_type) = entry.file_type() {
        file_type.is_dir()
    } else {
        false
    }
}

/// scan a directory recursively and access with iterator
pub struct DirIterator(Vec<fs::ReadDir>);

impl DirIterator {
    /// Scan current directory and return result as iterator
    pub fn new() -> Result<Self, io::Error> {
        Ok(Self(vec![fs::read_dir(env::current_dir()?)?]))
    }
    /// Scan given `path`` and return result as iterator
    pub fn from_path(path: impl AsRef<path::Path>) -> Result<Self, io::Error> {
        Ok(Self(vec![fs::read_dir(path)?]))
    }
}

impl Iterator for DirIterator {
    type Item = Result<fs::DirEntry, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let stack = &mut self.0;
        loop {
            if let Some(it) = stack.last_mut() {
                match it.next() {
                    Some(Ok(entry)) => match entry.file_type() {
                        Ok(file_type) => {
                            // Push new item on stack when the file entry is a directory
                            if file_type.is_dir() {
                                match fs::read_dir(entry.path()) {
                                    Ok(dir_entry) => stack.push(dir_entry),
                                    Err(err) => return Some(Err(err)),
                                }
                            }
                            return Some(Ok(entry));
                        }
                        Err(err) => return Some(Err(err)),
                    },
                    Some(Err(err)) => panic!("{err}"),
                    None => {
                        stack.pop()?;
                    }
                }
            } else {
                return None;
            }
        }
    }
}
