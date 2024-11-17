# dir-iterator

Iterator that recursively scans and filters files from a directory.

## Usage

### Installation

Start using this library by running the following command in your *cargo* project directory.

```sh
cargo add dir-iterator
```

### Read a Directory Recursively

Read a directory by using `DirIterator::build_from_path(`*path*`)`:

```rs
use dir_iterator::*;

fn main() {
    // read directory `src`
    DirIterator::build_from_path("src")
        // maybe maybe
        .expect("path not found")
        // print it out
        .for_each(|e| println!("{:?}", e.file_name()));
}
```

### Read Current Directory Recursively

Read current directory by using `DirIterator::build_current()` is a little shorter because it will panic if current directory does not exist or can't be retrieved.

```rs
use dir_iterator::*;

fn main() {
    // build a new iterator starting in the current directory
    DirIterator::build_current()
        // print each file name
        .for_each(|e| println!("{:?}", e.file_name()));
}
```

You may use `DirIterator::try_build_current()` to get errors instead of panic.

### Filter Result with Wildcards

Filter the result with wildcards by using `exclude(`*wildcard*`)` which generates a filter.

```rs
use dir_iterator::*;

fn main() {
    DirIterator::build_current()
        // filter all files which have extension `txt`
        .filter(exclude("*.txt"))
        .for_each(|e| println!("{:?}", e.file_name()));
}
```

### Ignore Folders When Scanning

To prevent some directories from being scanned at all you ca use `ignore(`*wildcard*`)`.

```rs
use dir_iterator::*;

fn main() {
    DirIterator::current()
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
```

Because `ignore()` parametrizes the following scan it must be placed in front of all `Iterator` trait methods and finished with `build()`.
