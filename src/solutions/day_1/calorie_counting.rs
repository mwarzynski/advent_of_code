use crate::parser::file;

const INPUT_FILE: &str = "./inputs/day_1.txt";

type Calories = u64;

#[derive(Debug)]
enum Event {
    NewElve,
    ElveGatherCalories(Calories),
}

fn process_line(line: String) -> Event {
    if line.is_empty() {
        Event::NewElve
    } else {
        match line.parse::<Calories>() {
            Ok(v) => Event::ElveGatherCalories(v),
            Err(e) => panic!("parse line \"{}\" as Calories failed: {}", line, e),
        }
    }
}

struct SantaTeam {
    elve_current: Calories,
    elves_top_n: Vec<Calories>,
}

impl SantaTeam {
    fn new(file_name: &str, n: usize) -> Self {
        let mut team = Self {
            elves_top_n: vec![0; n],
            elve_current: 0,
        };

        file::process(file_name, process_line, |event: Event| match event {
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
            }

            Event::ElveGatherCalories(calories) => {
                team.elve_current += calories;
            }
        });

        team
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

pub fn run() {
    let team = SantaTeam::new(INPUT_FILE, 3);
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
}

#[test]
fn top_1() {
    let team = SantaTeam::new(INPUT_FILE, 1);
    assert_eq!(67658, team.sum_top_n_calories(1).unwrap());
}

#[test]
fn top_3() {
    let team = SantaTeam::new(INPUT_FILE, 3);
    assert_eq!(200158, team.sum_top_n_calories(3).unwrap());
}
