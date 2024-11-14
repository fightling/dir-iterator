# dir-iterator

Iterator that recursively scans and filters files from a directory.

## Usage

### Installation

Start using this library by running the following command in your *cargo* project directory.

```toml
cargo add dir-iterator
```

### Read a Directory Recursively

```rs
use dir_iterator::*

fn main() {
    // create a new iterator starting in the current directory 
    DirIterator::new(".")
        // you will get this error if path was not found
        .expect("path not found")
        // while processing recursive dive multiple file system errors may occur.
        // flatten sorts them out
        .flatten()
        // print each file name
        .for_each(|e| println!("{:?}",e.file_name()));
}
```

### Filter Result with Wildcards

```rs
use dir_iterator::*

fn main() {
    // create a new iterator starting in the current directory 
    DirIterator::new(".")
        // you will get this error if path was not found
        .expect("path not found")
        // while processing recursive dive multiple file system errors may occur.
        // flatten sorts them out
        .flatten()
        // filter all files which have extension `txt`
        .filter(wildcard("*.txt"))
        // print each file name
        .for_each(|e| println!("{:?}",e.file_name()));
}
```
