use std::fs::File as FSFile;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(", ").unwrap();
        Ok(Point {
            x: x.strip_prefix("x=").unwrap().parse().unwrap(),
            y: y.strip_prefix("y=").unwrap().parse().unwrap(),
        })
    }
}

impl Point {
    pub fn distance(&self, to: &Point) -> i64 {
        // manhattan distance
        (self.x - to.x).abs() + (self.y - to.y).abs()
    }

    pub fn beacon_tuning_frequency(&self) -> i64 {
        // multiplying its x coordinate by 4000000 and then adding its y coordinate
        self.x * 4000000 + self.y
    }
}

#[derive(Debug)]
struct SensorMeasurement {
    sensor: Point,
    distance: i64,
}

impl FromStr for SensorMeasurement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sensor_str, beacon_str) = s.split_once(": ").unwrap();

        let sensor: Point = sensor_str.split_once("at ").unwrap().1.parse().unwrap();
        let beacon: Point = beacon_str.split_once("at ").unwrap().1.parse().unwrap();
        let distance = sensor.distance(&beacon);

        Ok(SensorMeasurement { sensor, distance })
    }
}

impl SensorMeasurement {
    pub fn sensored_range_for_y(&self, y: i64) -> Option<RangeInclusive<i64>> {
        let diff_y = i64::abs(self.sensor.y - y);
        if diff_y > self.distance {
            return None;
        }

        let x_diretion_length = self.distance - diff_y;

        return Some(RangeInclusive::new(
            self.sensor.x - x_diretion_length,
            self.sensor.x + x_diretion_length,
        ));
    }
}

struct Map {
    measurements: Vec<SensorMeasurement>,
}

impl Map {
    fn new() -> Map {
        Map {
            measurements: Vec::new(),
        }
    }

    pub fn add_measurement(&mut self, measurement: SensorMeasurement) {
        self.measurements.push(measurement);
    }

    pub fn find_tuning_frequency_of_distress_beacon(&self, search_bound: i64) -> i64 {
        // note: it should also be possible to find the tuning frequency by just looking at the edges of sensor
        // measurements, because the distress beacon should be somewhere around the edge
        // however, since we already have the 'sensored_range_for_y' from part1, let's reuse it and use brute force ftw
        for y in 0..search_bound {
            let ranges = self.sensored_range_for_y(y);
            match self.ranges_subtract(&ranges, RangeInclusive::new(0, search_bound)) {
                Some(x) => {
                    return Point { x, y }.beacon_tuning_frequency();
                }
                None => continue,
            }
        }
        0
    }

    fn ranges_subtract(
        &self,
        ranges: &Vec<RangeInclusive<i64>>,
        item: RangeInclusive<i64>,
    ) -> Option<i64> {
        let mut i = *item.start();
        for range in ranges {
            if *item.start() > *range.end() {
                continue;
            }
            if *range.start() > i {
                return Some(i);
            }
            let next_i = *range.end() + 1;
            if next_i > i && next_i <= *item.end() {
                i = *range.end() + 1;
            }
            if range.contains(item.end()) {
                break;
            }
        }
        return None;
    }

    pub fn count_fields_which_cannot_contain_a_beacon(&self, y: i64) -> i64 {
        let ranges = self.sensored_range_for_y(y);

        let mut count = 0;
        let mut i = *ranges[0].start();
        for range in ranges {
            if i > *range.end() {
                continue;
            }
            if *range.start() > i {
                i = *range.start();
            }
            let to_add = range.end() - i;

            count += to_add;
            i = *range.end();
        }

        count
    }

    fn sensored_range_for_y(&self, y: i64) -> Vec<RangeInclusive<i64>> {
        let mut ranges = Vec::new();
        for measurement in &self.measurements {
            match measurement.sensored_range_for_y(y) {
                Some(range) => ranges.push(range),
                None => (),
            }
        }
        ranges.sort_by(|a, b| return a.start().cmp(b.start()));
        return ranges;
    }
}

fn main() {
    let mut map = Map::new();

    let file = FSFile::open("./input.prod").expect("input file should exist");
    for line_wrapped in BufReader::new(file).lines() {
        let measurement = SensorMeasurement::from_str(&line_wrapped.unwrap()).unwrap();
        map.add_measurement(measurement);
    }

    println!(
        "{}",
        map.count_fields_which_cannot_contain_a_beacon(4000000)
    ); // 4737567
    println!("{}", map.find_tuning_frequency_of_distress_beacon(4000000));
}
