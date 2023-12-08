use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn power_of_two(e: u32) -> u32 {
    let mut v = 1;
    for _ in 0..e {
        v *= 2;
    }
    v
}

#[derive(Debug)]
struct Card {
    numbers_mine: HashSet<u32>,
    numbers_winning: HashSet<u32>,
}

impl Card {
    fn count_matching_numbers(&self) -> usize {
        let matching_numbers: HashSet<_> = self
            .numbers_mine
            .intersection(&self.numbers_winning)
            .collect();
        matching_numbers.len()
    }

    fn worth(&self) -> u32 {
        let count = self.count_matching_numbers();
        if count == 0 {
            return 0;
        }
        power_of_two((count - 1) as u32)
    }
}

impl FromStr for Card {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // Split the line into parts based on ':', ' ', and '|'
        let mut parts = line.split(|c| c == ':' || c == '|');

        // Extract the numbers for 'numbers_mine'
        let numbers_mine_str: &str = parts.nth(1).unwrap().trim();
        let numbers_mine: HashSet<u32> = numbers_mine_str
            .split(" ")
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().parse().unwrap())
            .collect();

        // Extract the numbers for 'numbers_winning'
        let numbers_winning_str: &str = parts.nth(0).unwrap().trim();
        let numbers_winning: HashSet<u32> = numbers_winning_str
            .split(" ")
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().parse().unwrap())
            .collect();

        Ok(Card {
            numbers_mine,
            numbers_winning,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input.prod").expect("input file should exist");
    let reader = BufReader::new(file);

    let mut cards: Vec<Card> = Vec::new();
    for line in reader.lines() {
        let card: Card = line.unwrap().parse()?;
        cards.push(card);
    }

    let part1: u32 = cards.iter().map(|c| c.worth()).sum();
    println!("{}", part1);

    let mut copies: Vec<u32> = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let base = *copies.get(i).unwrap();
        for j in 0..card.count_matching_numbers() {
            let k = i + j as usize + 1;
            if let Some(c) = copies.get_mut(k) {
                *c = *c + base;
            }
        }
    }
    let part2: u32 = copies.iter().sum();
    println!("{}", part2);

    Ok(())
}
