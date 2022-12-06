use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::string::ParseError;
use std::{error::Error, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
struct Operation {
    count: i32,
    from: usize,
    to: usize,
}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s_move, s_from_to) = s.split_once(" from ").unwrap();
        let (_, move_value) = s_move.split_once("move ").unwrap();
        let (from_value, to_value) = s_from_to.split_once(" to ").unwrap();
        Ok(Self {
            count: move_value.parse().unwrap(),
            from: from_value.parse().unwrap(),
            to: to_value.parse().unwrap(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct StacksLevel {
    stacks: Vec<char>,
}

impl FromStr for StacksLevel {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stacks_len = (s.len() + 1) / 4;
        let mut stacks: Vec<char> = Vec::new();
        for i in 0..stacks_len {
            stacks.push(s.chars().nth(i * 4 + 1).unwrap())
        }
        Ok(Self { stacks })
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Event {
    Init(StacksLevel),
    Move(Operation),
    LineNumber,
    Empty,
}

impl FromStr for Event {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Event::Empty);
        }
        if s.starts_with(" 1") {
            return Ok(Event::LineNumber);
        }
        if s.starts_with("move") {
            return Ok(Event::Move(Operation::from_str(s)?));
        }
        Ok(Event::Init(StacksLevel::from_str(s)?))
    }
}

struct StackCrates {
    stacks: Vec<VecDeque<char>>,
}

impl StackCrates {
    fn new() -> Self {
        Self { stacks: vec![] }
    }

    fn init_level(&mut self, level: StacksLevel) {
        if self.stacks.len() < level.stacks.len() {
            self.stacks.resize(level.stacks.len(), VecDeque::new())
        }
        for (stack, item) in level.stacks.iter().enumerate() {
            if *item != ' ' {
                self.stacks[stack].push_back(*item)
            }
        }
    }

    fn move_operation(&mut self, operation: Operation) {
        let mut count: i32 = i32::from(operation.count);
        let mut items = Vec::new();
        while count > 0 {
            items.push(self.stacks[operation.from - 1].pop_front().unwrap());
            count -= 1;
        }
        items.reverse();
        for item in items.iter() {
            self.stacks[operation.to - 1].push_front(*item);
        }
    }

    fn get_top_items(&self) -> String {
        let mut items = String::from("");
        for stack in self.stacks.iter() {
            match stack.front() {
                Some(v) => items.push(*v),
                _ => {}
            }
        }
        items
    }
}

const INPUT_FILE: &str = "./inputs/day_5.txt";

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut stack_crates = StackCrates::new();

    let file = File::open(INPUT_FILE).expect("input file should exist");
    let file_buffer = BufReader::new(file);
    for line in file_buffer.lines() {
        let event = Event::from_str(&line.unwrap()).unwrap();
        match event {
            Event::Init(stacks) => {
                stack_crates.init_level(stacks);
            }
            Event::Move(operation) => {
                stack_crates.move_operation(operation);
            }
            Event::Empty | Event::LineNumber => continue,
        }
    }

    println!("top crates: {}", stack_crates.get_top_items());
    Ok(())
}
