use fileops::finder::Finder;
use std::env;
use std::io::Error;

fn main() -> Result<(), Error> {
    let root = env::args().nth(1);
    let finder = Finder::new(root.as_deref())?;

    // Find all files in dir
    for file_path in finder.find() {
        println!("Found file: {:?}", file_path.file_name());
    }

    // Find specific files in dir
    for file_path in finder.filter_find("main") {
        println!("Found main file: {:?}", file_path.file_name());
    }

    Ok(())
}
