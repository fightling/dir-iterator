//! Directory Iterator

#![allow(dead_code)]

mod filter;
#[cfg(test)]
mod test;

use std::*;

/// We deal with std::io::Error
type Result<T> = io::Result<T>;

/// scan a directory recursively and access with iterator
pub struct DirIterator {
    /// current subdirectory (last is current folder)
    stack: Vec<fs::ReadDir>,
    /// configuration
    config: DirIteratorConfig,
}

impl DirIterator {
    /// Return an iterator builder aiming on current directory
    pub fn current() -> DirIteratorBuilder {
        Self::from_path(env::current_dir().expect("could not retrieve current dir"))
    }

    /// Scan current directory and return result as iterator
    pub fn build_current() -> impl Iterator<Item = fs::DirEntry> {
        Self::current().build().expect("path not found")
    }

    /// Return an iterator builder aiming on given directory
    pub fn from_path(path: impl AsRef<path::Path>) -> DirIteratorBuilder {
        DirIteratorBuilder {
            path: path.as_ref().to_path_buf(),
            ..Default::default()
        }
    }

    /// Scan given `path`` and return result as iterator
    pub fn build_from_path(
        path: impl AsRef<path::Path>,
    ) -> Result<impl Iterator<Item = fs::DirEntry>> {
        Self::from_path(path).build()
    }

    /// Create from `DirIteratorBuilder`
    fn from_builder(builder: DirIteratorBuilder) -> Result<Self> {
        Ok(Self {
            stack: vec![fs::read_dir(builder.path)?],
            config: builder.config,
        })
    }
}

impl Iterator for DirIterator {
    type Item = Result<fs::DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // get current `read_dir()` result on the stack
            if let Some(it) = self.stack.last_mut() {
                // get next item
                match it.next() {
                    // got one
                    Some(Ok(item)) => {
                        // check file type
                        match item.file_type() {
                            Ok(file_type) => {
                                // ignore folders if configured
                                if self.config.ignore.iter().any(|ignore| {
                                    ignore.is_match(item.file_name().as_encoded_bytes())
                                }) {
                                    continue;
                                }
                                // Push new item on stack when the file entry is a directory
                                if file_type.is_dir() {
                                    match fs::read_dir(item.path()) {
                                        Ok(dir_entry) => self.stack.push(dir_entry),
                                        Err(err) => return Some(Err(err)),
                                    }
                                }
                                // return next item
                                return Some(Ok(item));
                            }
                            // report error in item
                            Err(err) => return Some(Err(err)),
                        }
                    }
                    None => {
                        // finished with current `read_dir()` result
                        self.stack.pop()?;
                    }
                    err => return err,
                }
            } else {
                return None;
            }
        }
    }
}

/// Configuration of a `DirIterator`
#[derive(Default)]
struct DirIteratorConfig {
    /// If set do not scan directories which file name matches this wildcard
    ignore: Vec<wc::Wildcard<'static>>,
}

/// Builder for configuration of `DirIterator`
#[derive(Default)]
pub struct DirIteratorBuilder {
    /// Path to scan
    path: path::PathBuf,
    /// Scanner configuration
    config: DirIteratorConfig,
}

impl DirIteratorBuilder {
    /// finish configuration and build a `DirIterator`
    pub fn build(self) -> Result<impl Iterator<Item = fs::DirEntry>> {
        Ok(DirIterator::from_builder(self)?.flatten())
    }

    /// configures to ignore folders by wildcard
    pub fn ignore(mut self, wildcard: &'static str) -> Self {
        self.config
            .ignore
            .push(wc::Wildcard::new(wildcard.as_bytes()).expect("misformed wildcard"));
        self
    }
}
