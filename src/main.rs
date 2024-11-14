use std::*;

fn main() {
    let it = DirIterator::try_new(".").expect("path not found");

    it.for_each(|entry| println!("{}", entry.unwrap().file_name().to_string_lossy()));
}

struct DirIterator {
    stack: Vec<fs::ReadDir>,
}

impl DirIterator {
    fn try_new(path: impl AsRef<path::Path>) -> Result<Self, io::Error> {
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
