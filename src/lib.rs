//! Directory Iterator

#![allow(dead_code)]

mod filter;
#[cfg(test)]
mod test;

use std::*;

/// We deal with std::io::Error
type Result<T> = io::Result<T>;

#[derive(Default)]
/// scan a directory recursively and access with iterator
pub struct DirIterator {
    /// current subdirectory (last is current folder)
    stack: Vec<fs::ReadDir>,
    /// configuration
    config: DirIteratorConfig,
}

impl DirIterator {
    /// Return an iterator builder aiming on current directory
    pub fn current() -> Result<DirIteratorBuilder> {
        Self::from_path(env::current_dir()?)
    }

    /// Scan current directory and return iterator
    pub fn build_current() -> impl Iterator<Item = fs::DirEntry> {
        Self::current().expect("path not found").build()
    }

    /// Return an iterator builder aiming on given directory
    pub fn from_path(path: impl AsRef<path::Path>) -> Result<DirIteratorBuilder> {
        Ok(DirIteratorBuilder(Self {
            stack: vec![fs::read_dir(path)?],
            ..Default::default()
        }))
    }

    /// Scan given `path`` and return iterator
    pub fn build_from_path(
        path: impl AsRef<path::Path>,
    ) -> Result<impl Iterator<Item = fs::DirEntry>> {
        Ok(Self::from_path(path)?.build())
    }

    /// Create from `DirIteratorBuilder`
    fn from_builder(builder: DirIteratorBuilder) -> Self {
        builder.0
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
pub struct DirIteratorBuilder(DirIterator);

impl DirIteratorBuilder {
    /// finish configuration and build a `DirIterator`
    pub fn build(self) -> impl Iterator<Item = fs::DirEntry> {
        DirIterator::from_builder(self).flatten()
    }

    /// configures to ignore folders by wildcard
    pub fn ignore(mut self, wildcard: &'static str) -> Self {
        self.0
            .config
            .ignore
            .push(wc::Wildcard::new(wildcard.as_bytes()).expect("misformed wildcard"));
        self
    }
}
