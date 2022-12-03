use core::panic;
use std::collections::HashSet;
use std::error::Error;

use crate::parser::file;

const INPUT_FILE: &str = "./inputs/day_3.txt";

type Item = char;
type Items = String;

fn item_score(c: Item) -> u32 {
    // Lowercase item types a through z have priorities 1 through 26.
    // Uppercase item types A through Z have priorities 27 through 52.
    let mut v: u32 = c.into();
    if c.is_lowercase() {
        v -= 96;
    }
    if c.is_uppercase() {
        v -= 38;
    }
    v
}

fn items_score(items: Items) -> u32 {
    let mut score = 0;
    for item in items.chars() {
        score += item_score(item);
    }
    score
}

// Rucksack has two compartments which contain items.
struct Elve {
    // rucksack represents a bucket of items where each character corresponds to an item
    // items in the first half of the string belong to the first compartment
    // items in the second half of the string belong to the second compartment
    rucksack: String,
}

impl Elve {
    fn new(rucksack: String) -> Self {
        Self { rucksack }
    }

    fn find_common_items_in_both_compartments(&self) -> String {
        let mut compartment_first = HashSet::new();
        let mut common_items = HashSet::new();
        let rucksack_size = self.rucksack.len();
        for (i, item) in self.rucksack.chars().enumerate() {
            let is_first_compartment = i < rucksack_size / 2;
            if is_first_compartment {
                compartment_first.insert(item);
            } else if compartment_first.contains(&item) {
                common_items.insert(item);
            }
        }
        String::from_iter(common_items.iter().map(|c| c.clone()))
    }
}

struct ElvesGroup {
    elves: Vec<Elve>,
}

impl ElvesGroup {
    fn new() -> Self {
        Self { elves: vec![] }
    }

    fn add(&mut self, elve: Elve) {
        self.elves.push(elve)
    }

    fn can_find_badge(&self) -> bool {
        self.elves.len() == 3
    }

    fn find_badge(&self) -> Result<Item, Box<dyn Error>> {
        if !self.can_find_badge() {
            panic!("cannot find badge for elve group of size != 3")
        }

        let mut sets = Vec::new();
        for elve in self.elves.iter() {
            let mut set = HashSet::new();
            for item in elve.rucksack.chars() {
                set.insert(item);
            }
            sets.push(set)
        }

        let mut i: HashSet<char> = sets[0].intersection(&sets[1]).copied().collect();
        i = i.intersection(&sets[2]).copied().collect();

        if i.len() != 1 {
            panic!("there is more than one badge");
        }

        Ok(*i.iter().next().unwrap())
    }
}

pub fn part1() -> Result<(), Box<dyn Error>> {
    let mut score = 0;
    file::process(
        INPUT_FILE,
        |line: String| -> Result<Items, Box<dyn Error>> { Ok(Items::from(line)) },
        |rucksack: Items| -> Result<_, Box<dyn Error>> {
            let elve = Elve::new(rucksack);
            score += items_score(elve.find_common_items_in_both_compartments());
            Ok(())
        },
    )?;
    println!("score: {score}");
    Ok(())
}

pub fn part2() -> Result<(), Box<dyn Error>> {
    let mut score = 0;
    let mut elves_group = ElvesGroup::new();
    file::process(
        INPUT_FILE,
        |line: String| -> Result<Items, Box<dyn Error>> { Ok(Items::from(line)) },
        |rucksack: Items| -> Result<_, Box<dyn Error>> {
            elves_group.add(Elve::new(rucksack));
            if elves_group.can_find_badge() {
                score += item_score(elves_group.find_badge()?);
                elves_group = ElvesGroup::new();
            }
            Ok(())
        },
    )?;
    println!("score: {score}");
    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    part1()?;
    part2()?;
    Ok(())
}
