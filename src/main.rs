use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod lib;
use lib::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let lcov_file = &args[1];

    for line in lines_from_file(lcov_file) {
        match FileRecord::from_string(line) {
            Some(fr) => println!("{}", fr),
            None => continue,
        }
    }
}

pub fn lines_from_file<P>(fname: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let f = File::open(fname).expect("no such file");
    let buf = BufReader::new(f);
    buf.lines()
        .map(|l| l.expect("could not parse line"))
        .collect()
}
