use std::fs::File;
use std::io::{prelude::*, BufReader};
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    let file = File::open(&args.path).expect("File could not be opened");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(l) => {
                if l.contains(&args.pattern) {
                    println!("{}", l)
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}
