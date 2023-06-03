use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use fileops::searcher::Searches;

pub fn search_files(
    searcher: impl Searches,
    paths: Vec<PathBuf>,
    verbose: bool,
) -> Result<(), Error> {
    // Use the  provided searcher to search for matches
    for path in paths {
        let contents = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(ref e) if e.kind() == ErrorKind::InvalidData => {
                if verbose {
                    println!("Cannot read file: {:?}", path);
                    continue;
                } else {
                    continue;
                }
            }
            Err(e) => return Err(e),
        };

        for line_match in searcher.search(&contents) {
            println!("{:?}: {}", path, line_match);
        }
    }

    Ok(())
}
