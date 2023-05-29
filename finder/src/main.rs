use std::fs;
use std::io::{Error, ErrorKind};

use fileops::finder::Finder;
use fileops::searcher::Searcher;

use clap::Parser;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional path to operate on, defaults to CWD
    path: Option<String>,

    /// File pattern to filter results
    #[arg(short, long)]
    file_pattern: Option<String>,

    /// Search pattern find in result files
    #[arg(short, long)]
    search_pattern: Option<String>,

    /// Flag for case insensitive search
    #[arg(short = 'i', long)]
    case_insensitive: bool,

    /// Verbose output details unreadable files
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    // Grab finder values from the command line
    let finder = Finder::new(cli.path.as_deref())?;
    let file_pattern = cli.file_pattern.as_deref();

    for path in finder.find(file_pattern) {
        if let Some(query) = cli.search_pattern.as_deref() {
            let contents = match fs::read_to_string(&path) {
                Ok(c) => c,
                Err(ref e) if e.kind() == ErrorKind::InvalidData => {
                    let verbose = cli.verbose;
                    if verbose {
                        println!("Cannot read file: {:?}", path);
                        continue;
                    } else {
                        continue;
                    }
                }
                Err(e) => return Err(e),
            };

            let searcher = Searcher::new(query, &contents);
            let case_insensitive = cli.case_insensitive;

            for line_match in searcher.search(case_insensitive) {
                println!("{:?}: {}", path, line_match);
            }
        } else {
            println!("{:?}", path);
        }
    }

    Ok(())
}
