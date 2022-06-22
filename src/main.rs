use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod lib;
use lib::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let coverage_file = &args[1];

    if let Ok(lines) = read_lines(coverage_file) {
        for line in lines.flatten() {
            match FileCov::parse(&line) {
                Some(fr) => println!("{}", fr),
                None => continue,
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
