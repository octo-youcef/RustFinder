use std::env;
use std::io::Error;
use std::process;

use fileops::finder;

fn main() -> Result<(), Error> {
    // optional input directory
    let root = env::args().nth(1);
    let query = env::args().nth(2);

    // create the finder from the root directory
    let finder = match finder::Finder::new(root.as_deref()) {
        Ok(finder) => finder,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(exitcode::DATAERR);
        }
    };

    for file_path in finder.find(query.as_deref()) {
        println!("{:?}", file_path.as_path());
    }

    Ok(())
}
