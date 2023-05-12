# FileOps
A collection of file and file-system operations

## TODO
 - [x] find files recursively from a specified directory
 - [ ] search file contents for \<search pattern\>
    - [ ] case insensitive/sensitive search
    - [ ] regex search

## Public API
### finder::Finder
The `Finder` struct holds a file path, and recursively finds all file objects from that path. Optionally file names can be filtered using the `filter_find` method.

```rust
pub struct Finder { }

impl Finder {
    pub fn new(root: Option<&str>) -> Finder { }
    
    pub fn find(&self) -> Vec<PathBuf> { }
        // Recursively find files from the finder root

    pub fn filter_find(&self) -> Vec<PathBuf> { }
        // Recursively find files from the finder root
        //filtering to filenames containing some value
```
