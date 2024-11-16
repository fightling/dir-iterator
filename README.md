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
    // create a new iterator starting in the current directory 
    DirIterator::current()
        // this is academic
        .expect("could not retrieve current dir")
        // build iterator
        .build()
        // you will get this error if path was not found
        .expect("path not found")
        // print each file name
        .for_each(|e| println!("{:?}",e.file_name()));
}
```

### Filter Result with Wildcards

```rs
use dir_iterator::*

fn main() {
    DirIterator::current()
        .expect("could not retrieve current dir")
        .build()
        .expect("path not found")
        // filter all files which have extension `txt`
        .filter(wildcard("*.txt"))
        .for_each(|e| println!("{:?}",e.file_name()));
}
```

### Ignore Folders When Scanning

```rs
use dir_iterator::*

fn main() {
    DirIterator::current()
        .expect("could not retrieve current dir")
        // ignore target directory
        .ignore("target")
        .build()
        .expect("path not found")
        .for_each(|e| println!("{:?}", e.path()));
}
```
