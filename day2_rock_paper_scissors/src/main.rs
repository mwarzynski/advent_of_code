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

#[derive(Debug, Clone, Copy, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn new(v: &str) -> Self {
        match v {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic!("undefined shape"),
        }
    }

    fn score(&self) -> u64 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum RoundOutcome {
    Lost,
    Draw,
    Won,
}

impl RoundOutcome {
    fn new(v: &str) -> Self {
        match v {
            "X" => RoundOutcome::Lost,
            "Y" => RoundOutcome::Draw,
            "Z" => RoundOutcome::Won,
            _ => panic!("undefined round outcome"),
        }
    }

    fn score(&self) -> u64 {
        match self {
            RoundOutcome::Draw => 3,
            RoundOutcome::Won => 6,
            _ => 0,
        }
    }
}

#[derive(Debug)]
struct Round {
    player1: Shape,
    player2: Shape,
}

impl Round {
    fn new(player1: Shape, player2: Shape) -> Self {
        Self {
            player1: player1,
            player2: player2,
        }
    }

    pub fn score(&self) -> u64 {
        self.player1_outcome().score() + self.player1.score()
    }

    fn player1_outcome(&self) -> RoundOutcome {
        match self.player1 {
            Shape::Paper => match self.player2 {
                Shape::Paper => RoundOutcome::Draw,
                Shape::Rock => RoundOutcome::Won,
                Shape::Scissors => RoundOutcome::Lost,
            },
            Shape::Rock => match self.player2 {
                Shape::Paper => RoundOutcome::Lost,
                Shape::Rock => RoundOutcome::Draw,
                Shape::Scissors => RoundOutcome::Won,
            },
            Shape::Scissors => match self.player2 {
                Shape::Paper => RoundOutcome::Won,
                Shape::Rock => RoundOutcome::Lost,
                Shape::Scissors => RoundOutcome::Draw,
            },
        }
    }
}

#[derive(Debug)]
struct RoundPuzzle {
    player2: Shape,
    expected_outcome: RoundOutcome,
}

impl RoundPuzzle {
    fn new(player2: Shape, expected_outcome: RoundOutcome) -> Self {
        Self {
            player2: player2,
            expected_outcome: expected_outcome,
        }
    }

    fn guess_round(&self) -> Round {
        let player1: Shape = match self.expected_outcome {
            RoundOutcome::Draw => match self.player2 {
                _ => self.player2,
            },
            RoundOutcome::Lost => match self.player2 {
                Shape::Paper => Shape::Rock,
                Shape::Rock => Shape::Scissors,
                Shape::Scissors => Shape::Paper,
            },
            RoundOutcome::Won => match self.player2 {
                Shape::Paper => Shape::Scissors,
                Shape::Rock => Shape::Paper,
                Shape::Scissors => Shape::Rock,
            },
        };
        Round {
            player1: player1,
            player2: self.player2,
        }
    }
}

fn part1() -> Result<u64, Box<dyn Error>> {
    let mut score_part1: u64 = 0;
    process(
        INPUT_FILE,
        |line: String| -> Result<Round, Box<dyn Error>> {
            let line_components: Vec<&str> = line.split(" ").collect();
            Ok(Round::new(
                Shape::new(line_components[1]),
                Shape::new(line_components[0]),
            ))
        },
        |round: Round| {
            score_part1 += round.score();
            Ok(())
        },
    )?;
    Ok(score_part1)
}

fn part2() -> Result<u64, Box<dyn Error>> {
    let mut score_part2: u64 = 0;
    process(
        INPUT_FILE,
        |line: String| -> Result<RoundPuzzle, Box<dyn Error>> {
            let line_components: Vec<&str> = line.split(" ").collect();
            Ok(RoundPuzzle::new(
                Shape::new(line_components[0]),
                RoundOutcome::new(line_components[1]),
            ))
        },
        |round_puzzle: RoundPuzzle| {
            score_part2 += round_puzzle.guess_round().score();
            Ok(())
        },
    )?;
    Ok(score_part2)
}

const INPUT_FILE: &str = "./input.prod";

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("score: {}", part1()?);
    println!("score: {}", part2()?);
    Ok(())
}

#[test]
fn calculate_score() {
    assert_eq!(13484, part1().unwrap())
}

#[test]
fn guess_shape() {
    assert_eq!(13433, part2().unwrap())
}
