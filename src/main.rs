use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

mod lib;
use lib::FileCov;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("missing command argument, a coverage file is required.");
        std::process::exit(1);
    }

    let coverage_file = &args[1];

    if let Ok(lines) = read_lines(coverage_file) {
        for line in lines.flatten() {
            match FileCov::try_from(line.as_str()) {
                Ok(v) => println!("{}", v),
                Err(_) => continue,
            }
        }
    } else {
        eprintln!("error reading coverage file: {}", coverage_file);
        std::process::exit(1);
    }
}

// TODO: a better read_lines fn, and also support stdin
fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
