use core::panic;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File as FSFile;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug)]
struct ParseError;

enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("noop") {
            return Ok(Instruction::Noop);
        } else if s.starts_with("addx") {
            let (_, number) = s.split_once(" ").unwrap();
            let v: i32 = number.parse().unwrap();
            return Ok(Instruction::Addx(v));
        }
        panic!("unable to parse line");
    }
}

struct CPU {
    cycle: u32,
    reg_x: i32,

    breakpoints: HashSet<u32>,
    breakpoint_register: HashMap<u32, i32>,
}

impl CPU {
    fn new() -> Self {
        Self {
            cycle: 0,
            reg_x: 1,
            breakpoints: HashSet::new(),
            breakpoint_register: HashMap::new(),
        }
    }

    pub fn set_breakpoints(&mut self, breakpoints: Vec<u32>) {
        self.breakpoints.clear();
        for bp in breakpoints {
            self.breakpoints.insert(bp);
        }
    }

    pub fn exec(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => {
                self.cycle();
            }
            Instruction::Addx(v) => {
                self.cycle();
                self.cycle();
                self.reg_x += v;
            }
        }
    }

    fn cycle(&mut self) {
        // println!("cycle {}: {}", self.cycle, self.X);
        self.cycle += 1;

        if self.breakpoints.contains(&self.cycle) {
            self.breakpoint_register.insert(self.cycle, self.reg_x);
        }
    }

    pub fn signal_strength(cycle: &u32, reg_x: &i32) -> i32 {
        return (*cycle as i32) * reg_x;
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut cpu = CPU::new();
    cpu.set_breakpoints(vec![20, 60, 100, 140, 180, 220]);

    let file = FSFile::open("./inputs/day_10.txt").expect("input file should exist");
    for line_wrapped in BufReader::new(file).lines() {
        let instruction = Instruction::from_str(&line_wrapped.unwrap()).unwrap();
        cpu.exec(instruction);
    }

    let mut signal_strengths_sum = 0;
    for (cycle, reg_x) in cpu.breakpoint_register.iter() {
        let signal_strength = CPU::signal_strength(cycle, reg_x);
        signal_strengths_sum += signal_strength;
    }
    println!("sum: {}", signal_strengths_sum);

    Ok(())
}
