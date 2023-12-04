use std::fs::File as FSFile;
use std::io::{BufRead, BufReader};

struct Coordinates {
    values: Vec<i32>,
}

impl Coordinates {
    fn new(values: Vec<i32>) -> Self {
        Self { values }
    }
}

fn main() {
    let mut vs = Vec::new();
    let file = FSFile::open("./input.dev").expect("input file should exist");
    for line_wrapped in BufReader::new(file).lines() {
        vs.push(line_wrapped.unwrap().parse::<i32>().unwrap());
    }

    let mut coordinates = Coordinates::new(vs);
}
