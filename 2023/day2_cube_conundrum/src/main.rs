use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for CubeSet {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cube_set = CubeSet {
            red: 0,
            green: 0,
            blue: 0,
        };

        for x in s.split(',') {
            let x_parts: Vec<&str> = x.trim().split(' ').collect();
            let n: u32 = x_parts[0].trim().parse().map_err(|_| "invalid cube num")?;
            match x_parts[1].trim() {
                "red" => cube_set.red = n,
                "green" => cube_set.green = n,
                "blue" => cube_set.blue = n,
                _ => return Err("invalid cube color"),
            }
        }

        Ok(cube_set)
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    cube_sets: Vec<CubeSet>,
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();

        let game_id: i32 = parts
            .get(0)
            .ok_or("no game id")?
            .trim_start_matches("Game ")
            .parse()
            .map_err(|_| "invalid game id")?;

        let mut cube_sets: Vec<CubeSet> = Vec::new();
        for cs in parts.iter().skip(1) {
            for c in cs.split(';') {
                match CubeSet::from_str(c.trim()) {
                    Ok(cube_set) => cube_sets.push(cube_set),
                    Err(e) => return Err(e),
                }
            }
        }

        Ok(Self {
            id: game_id,
            cube_sets,
        })
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Game {}: ", self.id)?;
        for cube_set in self.cube_sets.iter() {
            write!(
                f,
                "red {}: green {}, blue {}; \n",
                cube_set.red, cube_set.green, cube_set.blue,
            )?;
        }
        Ok(())
    }
}

fn part1_sum_game_ids_of_possible_games(games: &Vec<Game>, cube_set_max: CubeSet) -> i32 {
    let games_possible = games.iter().filter(|game| {
        for cube_set in game.cube_sets.iter() {
            if cube_set.blue > cube_set_max.blue
                || cube_set.red > cube_set_max.red
                || cube_set.green > cube_set_max.green
            {
                return false;
            }
        }
        true
    });
    games_possible.into_iter().map(|game| game.id).sum()
}

fn part2_sum_powers_of_minimal_sets(games: &Vec<Game>) -> u32 {
    let mut result = 0;
    for game in games.iter() {
        let max_red = game.cube_sets.iter().map(|cs| cs.red).max().unwrap_or(0);
        let max_blue = game.cube_sets.iter().map(|cs| cs.blue).max().unwrap_or(0);
        let max_green = game.cube_sets.iter().map(|cs| cs.green).max().unwrap_or(0);
        result += max_red * max_green * max_blue;
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input.prod").expect("input file should exist");
    let file_buffer = BufReader::new(file);

    let mut games: Vec<Game> = Vec::new();
    for line in file_buffer.lines() {
        match line {
            Ok(line) => {
                let game: Game = line.parse()?;
                games.push(game);
            }
            Err(e) => return Err(Box::new(e)),
        }
    }

    println!(
        "{}",
        part1_sum_game_ids_of_possible_games(
            &games,
            CubeSet {
                red: 12,
                green: 13,
                blue: 14,
            },
        )
    );

    println!("{}", part2_sum_powers_of_minimal_sets(&games));

    Ok(())
}

