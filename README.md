# dir-iterator

Iterator that recursively scans and filters files from a directory.

## Usage

### Installation

Start using this library by running the following command in your *cargo* project directory.

```sh
cargo add dir-iterator
```

### Read a Directory Recursively

```rs
use dir_iterator::*

fn main() {
    // build a new iterator starting in the current directory
    DirIterator::build_current()
        // print each file name
        .for_each(|e| println!("{:?}", e.file_name()));
}
```

### Filter Result with Wildcards

```rs
use dir_iterator::*

fn main() {
    DirIterator::build_current()
        // filter all files which have extension `txt`
        .filter(exclude("*.txt"))
        .for_each(|e| println!("{:?}", e.file_name()));
}
```

### Ignore Folders When Scanning

```rs
use dir_iterator::*

fn main() {
    DirIterator::current()
        // ignore target directory
        .ignore("target")
        // ignore all hidden directories
        .ignore(".*")
        // build iterator
        .build()
        .expect("path not found")
        // exclude all hidden files
        .filter(exclude(".*"))
        .for_each(|e| println!("{:?}", e.path()));
}
```
