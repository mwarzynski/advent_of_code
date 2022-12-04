use std::num::ParseIntError;
use std::{error::Error, str::FromStr};

use crate::parser::file;

const INPUT_FILE: &str = "./inputs/day_4.txt";

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
    file::process(
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
    file::process(
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

pub fn run() -> Result<(), Box<dyn Error>> {
    part1()?;
    part2()?;
    Ok(())
}
