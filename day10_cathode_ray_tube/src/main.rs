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

struct CRT {
    wide: usize,
    screen: Vec<Vec<char>>,

    screen_i_wide: usize,
    screen_i_high: usize,
}

impl CRT {
    fn new(wide: usize, high: usize) -> Self {
        let mut screen = Vec::new();
        for i in 0..high {
            screen.push(Vec::new());
            for _ in 0..wide {
                screen.get_mut(i as usize).unwrap().push('.');
            }
        }
        Self {
            screen,
            wide,
            screen_i_high: 0,
            screen_i_wide: 0,
        }
    }

    // Sprite is 3 pixels wide, and the X register sets the horizontal position of the middle of that sprite.
    pub fn draw_pixel(&mut self, sprite_middle_position: i32) {
        let mut pixel = '.';
        if i32::abs(self.screen_i_wide as i32 - sprite_middle_position) < 2 {
            pixel = '#';
        }
        self.screen[self.screen_i_high][self.screen_i_wide] = pixel;
        self.screen_i_wide += 1;
        if self.screen_i_wide == self.wide {
            self.screen_i_wide = 0;
            self.screen_i_high += 1;
        }
    }

    pub fn print(&self) {
        for row in self.screen.iter() {
            for col in row {
                print!("{}", col);
            }
            println!();
        }
    }
}

struct CPU {
    cycle: u32,
    reg_x: i32,

    crt: CRT,

    breakpoints: HashSet<u32>,
    breakpoint_register: HashMap<u32, i32>,
}

impl CPU {
    fn new() -> Self {
        Self {
            cycle: 1,
            reg_x: 1,
            breakpoints: HashSet::new(),
            breakpoint_register: HashMap::new(),
            crt: CRT::new(40, 6),
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

        self.crt.draw_pixel(self.reg_x);

        if self.breakpoints.contains(&self.cycle) {
            self.breakpoint_register.insert(self.cycle, self.reg_x);
        }

        self.cycle += 1;
    }

    #[allow(dead_code)]
    pub fn signal_strength(cycle: &u32, reg_x: &i32) -> i32 {
        return (*cycle as i32) * reg_x;
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut cpu = CPU::new();
    cpu.set_breakpoints(vec![20, 60, 100, 140, 180, 220]);

    let file = FSFile::open("./input.prod").expect("input file should exist");
    for line_wrapped in BufReader::new(file).lines() {
        let instruction = Instruction::from_str(&line_wrapped.unwrap()).unwrap();
        cpu.exec(instruction);
    }

    // part1
    let mut signal_strengths_sum = 0;
    for (cycle, reg_x) in cpu.breakpoint_register.iter() {
        let signal_strength = CPU::signal_strength(cycle, reg_x);
        signal_strengths_sum += signal_strength;
    }
    println!("sum: {}", signal_strengths_sum); // 12540

    // part2
    cpu.crt.print();

    Ok(())
}
