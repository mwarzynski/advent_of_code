use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
fn part1_line_num(line: &String) -> u32 {
    let char_to_digit = |c: char| c.to_digit(10);

    let a: u32 = line.chars().find_map(|c| char_to_digit(c)).unwrap_or(0);
    let b: u32 = line
        .chars()
        .rev()
        .find_map(|c| char_to_digit(c))
        .unwrap_or(0);
    return 10 * a + b;
}

const DIGITS_STR: [(&'static str, u32); 10] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("zero", 0),
];

fn part2_str_digit(line: &String, i: usize) -> Option<u32> {
    if let Some(c) = line.get(i..).unwrap().chars().nth(0) {
        if let Some(v) = c.to_digit(10) {
            return Some(v);
        }
    }
    let substr = line.get(i..).unwrap_or("");
    for digit_str in DIGITS_STR {
        if substr.starts_with(digit_str.0) {
            return Some(digit_str.1);
        }
    }
    None
}

fn part2_line_num(line: &String) -> u32 {
    let a: u32 = line
        .clone()
        .chars()
        .enumerate()
        .find_map(|(i, _)| part2_str_digit(line, i))
        .unwrap_or(0);
    let b: u32 = line
        .clone()
        .chars()
        .enumerate()
        .find_map(|(i, _)| part2_str_digit(line, line.len() - i - 1))
        .unwrap_or(0);
    10 * a + b
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input.prod").expect("input file should exist");
    let file_buffer = BufReader::new(file);

    let mut sum = 0;
    for line in file_buffer.lines() {
        match line {
            Ok(line) => sum += part2_line_num(&line),
            Err(e) => return Err(Box::new(e)),
        }
    }
    println!("sum: {}", sum);

    Ok(())
}
