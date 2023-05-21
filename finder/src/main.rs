use std::fs;
use std::io::Error;

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
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    // Grab finder values from the command line
    let finder = Finder::new(cli.path.as_deref())?;
    let file_pattern = cli.file_pattern.as_deref();

    for path in finder.find(file_pattern) {
        if let Some(query) = cli.search_pattern.as_deref() {
            let contents = fs::read_to_string(&path)?;
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
