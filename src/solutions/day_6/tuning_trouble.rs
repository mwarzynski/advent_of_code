use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_FILE: &str = "./inputs/day_6.txt";

const MARKER_SIZE: usize = 14; // part1: 4

fn datastream_find_start_marker(datastream: &str) -> Option<usize> {
    let mut marker = VecDeque::new();

    for (i, data) in datastream.chars().enumerate() {
        marker.push_back(data);
        if marker.len() == MARKER_SIZE {
            let set: HashSet<char, RandomState> = HashSet::from_iter(marker.iter().cloned());
            if set.len() == MARKER_SIZE {
                return Some(i + 1);
            }
            marker.pop_front();
        }
    }

    None
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = File::open(INPUT_FILE).expect("input file should exist");
    let file_buffer = BufReader::new(file);
    for line_result in file_buffer.lines() {
        match line_result {
            Ok(line) => match datastream_find_start_marker(&line) {
                Some(marker_i) => {
                    println!("marker: {}", marker_i);
                    return Ok(());
                }
                None => println!("marker not found"),
            },
            Err(e) => panic!("{e}"),
        }
    }
    Ok(())
    // part1: 1175
    // part2: 3217
}
