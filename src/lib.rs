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

/// scan a directory recursively and access with iterator
pub struct DirIterator {
    /// stack representing the current directory dive
    stack: Vec<fs::ReadDir>,
}

impl DirIterator {
    /// Create new iterator from a `path` to scan in
    pub fn new(path: impl AsRef<path::Path>) -> Result<Self, io::Error> {
        Ok(Self {
            stack: vec![fs::read_dir(path)?],
        })
    }
}

impl Iterator for DirIterator {
    type Item = Result<fs::DirEntry, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(it) = self.stack.last_mut() {
                match it.next() {
                    Some(Ok(entry)) => match entry.file_type() {
                        Ok(file_type) => {
                            if file_type.is_file() {
                                return Some(Ok(entry));
                            } else {
                                self.stack.push(fs::read_dir(entry.path()).expect(""))
                            }
                        }
                        Err(err) => return Some(Err(err)),
                    },
                    Some(Err(err)) => panic!("{err}"),
                    None => {
                        self.stack.pop()?;
                    }
                }
            } else {
                return None;
            }
        }
    }
}
