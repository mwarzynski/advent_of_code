use std::num::ParseIntError;
use std::str::FromStr;

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn process<F, G, Event>(
    file_name: &str,
    mut process_line: F,
    mut process_event: G,
) -> Result<(), Box<dyn Error>>
where
    F: FnMut(String) -> Result<Event, Box<dyn Error>>,
    G: FnMut(Event) -> Result<(), Box<dyn Error>>,
{
    let file = File::open(file_name).expect("input file should exist");
    let file_buffer = BufReader::new(file);
    for line in file_buffer.lines() {
        match line {
            Ok(l) => process_event(process_line(l)?)?,
            Err(e) => panic!("cannot read line from filepath={}: {}", file_name, e),
        }
    }
    Ok(())
}

const INPUT_FILE: &str = "./input.prod";

struct SectionRange {
    a: u32,
    b: u32,
}

impl SectionRange {
    fn contains(&self, other: &SectionRange) -> bool {
        self.a <= other.a && self.b >= other.b
    }

    fn overlap(&self, other: &SectionRange) -> bool {
        (self.a <= other.a && self.b >= other.a)
            || (self.b >= other.b && self.a <= other.b)
            || (self.a >= other.a && self.b <= other.b)
            || (other.a >= self.a && other.b <= self.b)
    }
}

impl FromStr for SectionRange {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('-').unwrap();
        Ok(SectionRange {
            a: a.parse::<u32>()?,
            b: b.parse::<u32>()?,
        })
    }
}

enum Event {
    ElveSectionAssignment(SectionRange, SectionRange),
}

impl FromStr for Event {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (elve_1_sections, elve_2_sections) = s.split_once(',').unwrap();
        Ok(Self::ElveSectionAssignment(
            SectionRange::from_str(elve_1_sections)?,
            SectionRange::from_str(elve_2_sections)?,
        ))
    }
}

fn part1() -> Result<(), Box<dyn Error>> {
    let mut counter = 0;
    process(
        INPUT_FILE,
        |line: String| Ok(Event::from_str(&line)?),
        |event: Event| match event {
            Event::ElveSectionAssignment(section1, section2) => {
                if section1.contains(&section2) || section2.contains(&section1) {
                    counter += 1;
                }
                Ok(())
            }
        },
    )?;
    println!("sections: {counter}");
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let mut counter = 0;
    process(
        INPUT_FILE,
        |line: String| Ok(Event::from_str(&line)?),
        |event: Event| match event {
            Event::ElveSectionAssignment(section1, section2) => {
                if section1.overlap(&section2) {
                    counter += 1;
                }
                Ok(())
            }
        },
    )?;
    println!("sections: {counter}");
    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    part1()?;
    part2()?;
    Ok(())
}
