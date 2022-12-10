use std::fs::File as FSFile;
use std::io::{BufRead, BufReader};

fn main() {
    let file = FSFile::open("./input.dev").expect("input file should exist");
    for line_wrapped in BufReader::new(file).lines() {
        println!("{}", line_wrapped.unwrap());
    }
}
