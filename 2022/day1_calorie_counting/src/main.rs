use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_FILE: &str = "./input.prod";

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

type Calories = u64;

#[derive(Debug)]
enum Event {
    NewElve,
    ElveGatherCalories(Calories),
}

fn process_line(line: String) -> Result<Event, Box<dyn Error>> {
    if line.is_empty() {
        Ok(Event::NewElve)
    } else {
        match line.parse::<Calories>() {
            Ok(v) => Ok(Event::ElveGatherCalories(v)),
            Err(e) => Err(Box::new(e)),
        }
    }
}

struct SantaTeam {
    elve_current: Calories,
    elves_top_n: Vec<Calories>,
}

impl SantaTeam {
    fn new(file_name: &str, n: usize) -> Result<Self, Box<dyn Error>> {
        let mut team = Self {
            elves_top_n: vec![0; n],
            elve_current: 0,
        };

        process(file_name, process_line, |event: Event| match event {
            Event::NewElve => {
                let elve_worst = team
                    .elves_top_n
                    .last_mut()
                    .expect("team must have at least one elve");
                // If the worst Elve of our TopN group gathered less Calories than current Elve:
                if *elve_worst < team.elve_current {
                    // Replace the 'worst Elve' with the current Elve and sort the topN again.
                    *elve_worst = team.elve_current;
                    team.elves_top_n.sort_by(|a, b| b.cmp(a));
                }
                team.elve_current = 0;
                Ok(())
            }

            Event::ElveGatherCalories(calories) => {
                team.elve_current += calories;
                Ok(())
            }
        })?;

        Ok(team)
    }

    fn sum_top_n_calories(&self, n: usize) -> Result<Calories, &'static str> {
        if n > self.elves_top_n.len() {
            return Err(
                "n cannot be larger than number of top team passed in the SantaTeam::new constructor",
            );
        }
        let mut calories: Calories = 0;
        for i in 0..=n - 1 {
            calories += self.elves_top_n[i];
        }
        Ok(calories)
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let team = SantaTeam::new(INPUT_FILE, 3)?;
    println!(
        "team.sum_top_n_calories(1): {}",
        team.sum_top_n_calories(1)
            .expect("top 1 elve must exist accordingly to the task specification")
    );
    println!(
        "team.sum_top_n_calories(3): {}",
        team.sum_top_n_calories(3)
            .expect("top 3 elves must exist accordingly to the task specification")
    );
    Ok(())
}

#[test]
fn top_1() {
    let team = SantaTeam::new(INPUT_FILE, 1).unwrap();
    assert_eq!(67658, team.sum_top_n_calories(1).unwrap());
}

#[test]
fn top_3() {
    let team = SantaTeam::new(INPUT_FILE, 3).unwrap();
    assert_eq!(200158, team.sum_top_n_calories(3).unwrap());
}
