# FileOps
A collection of file and file-system operations

## TODO
 - [ ] regex search

## Public API
### finder::Finder
The `Finder` struct holds a file path, and recursively finds all file objects from that path. Optionally file names can be filtered using the `filter_find` method.

```rust
pub struct Finder { 
    pub path: &Path,
}

impl Finder {
    pub fn new(root: Option<&str>) -> Result<Finder, std::io::Error> { }
    
    pub fn find(&self, query: Option<&str>) -> Vec<PathBuf> { }
        /// Recursively find files from the finder root
        /// optionally filtering to filenames containing some value
```

### searcher::Searcher
pub struct Searcher {
    query: &str,
    contents: &str,
}

impl Searcher {
    pub fn new(query, contents) -> Searcher { }

    pub fn search(&self, case_insensitive: bool) -> Vec<&str> { }
        /// Search self.contents for self.query
        /// optionally ignoring case
}
