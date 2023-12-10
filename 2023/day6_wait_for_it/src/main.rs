use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn race_ways_to_beat_record(race_time: i64, race_distance_record: i64) -> i64 {
    // race_time: total time of the race (includes speed_time and button_time)
    // race_distance_record: record distance established by previous opponents, goal is to achieve
    //     longer distance to win the race
    // button_time: duration while button is pressed, which increases speed of the boat
    // speed_time: duration while boat is travelling with speed defined by the 'button_time'

    // assumptions:
    // race_time > button_time > 0
    // race_time > speed_time > 0

    // button_time + speed_time = race_time
    // button_time * speed_time > race_distance_record
    // ^^ this is a quadratic equation, which we can solve in O(1)

    // speed_time = (race_time - button_time)
    // (race_time - button_time) * button_time > race_distance_record
    // race_time * button_time - button_time**2 > race_distance_record
    // -button_time**2 + race_time*button_time - race_distance_record > 0

    let delta = (race_time.pow(2) - 4 * race_distance_record) as f64;

    let button_time_max = (((race_time as f64) + delta.sqrt()) / 2.0).floor() as i64;
    let button_time_min = (((race_time as f64) - delta.sqrt()) / 2.0).ceil() as i64;

    return button_time_max - button_time_min + 1;
}

fn part1(filename: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename).expect("input file should exist");
    let file_buffer = BufReader::new(file);

    let parse_numbers = |line: String, prefix: &str| -> Vec<i64> {
        line.trim_start_matches(prefix)
            .split(" ")
            .map(|v| v.trim())
            .filter(|v| !v.is_empty())
            .map(|v| v.parse().unwrap())
            .collect()
    };

    let mut lines = file_buffer.lines();
    let times = parse_numbers(lines.nth(0).unwrap_or(Ok(String::new())).unwrap(), "Time: ");
    let distance = parse_numbers(
        lines.nth(0).unwrap_or(Ok(String::new())).unwrap(),
        "Distance: ",
    );
    let races: Vec<(&i64, &i64)> = times.iter().zip(distance.iter()).collect();

    let solution = races
        .iter()
        .map(|r| race_ways_to_beat_record(*r.0, *r.1))
        .reduce(|a, b| a * b)
        .unwrap_or(0);
    println!("{:?}", solution);
    Ok(())
}

fn part2(filename: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename).expect("input file should exist");
    let file_buffer = BufReader::new(file);

    let parse_number = |line: String, prefix| -> i64 {
        line.trim_start_matches(prefix)
            .replace(" ", "")
            .parse()
            .unwrap_or(0)
    };

    let mut lines = file_buffer.lines();
    let race_time = parse_number(lines.next().unwrap().unwrap(), "Time: ");
    let race_distance_record = parse_number(lines.next().unwrap().unwrap(), "Distance: ");

    let solutions = race_ways_to_beat_record(race_time, race_distance_record);
    println!("{}", solutions);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "input.prod";
    part1(filename)?;
    part2(filename)?;

    Ok(())
}
