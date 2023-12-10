use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::thread;

#[derive(Debug)]
struct RangeMap {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl RangeMap {
    fn convert(&self, l: u64) -> Option<u64> {
        if self.source_range_start <= l && l < self.source_range_start + self.range_length {
            let diff = l - self.source_range_start;
            return Some(self.destination_range_start + diff);
        }
        None
    }
}

impl FromStr for RangeMap {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split(" ").collect();

        if parts.len() != 3 {
            return Err("invalid range map line");
        }

        let destination_range_start: u64 = parts.get(0).unwrap().trim().parse().unwrap();
        let source_range_start: u64 = parts.get(1).unwrap().trim().parse().unwrap();
        let range_length: u64 = parts.get(2).unwrap().trim().parse().unwrap();

        Ok(Self {
            destination_range_start,
            source_range_start,
            range_length,
        })
    }
}

#[derive(Debug)]
struct RangeMaps {
    maps: Vec<RangeMap>,
}

impl RangeMaps {
    fn convert(&self, l: u64) -> u64 {
        for map in self.maps.iter() {
            if let Some(lm) = map.convert(l) {
                return lm;
            }
        }
        return l;
    }
}

impl FromStr for RangeMaps {
    type Err = &'static str;

    fn from_str(lines: &str) -> Result<Self, Self::Err> {
        let mut maps: Vec<RangeMap> = Vec::new();
        for line in lines.split("\n") {
            if let Ok(map) = line.parse() {
                maps.push(map);
            }
        }
        Ok(Self { maps })
    }
}

// Almanac (also spelled almanack and almanach) is a regularly published listingof
// of a set of current information about one or multiple subjects.
// Source: Wikipedia
#[derive(Debug)]
struct Almanac {
    seeds: Vec<(u64, u64)>,
    seed_to_soil: RangeMaps,
    soil_to_fertilizer: RangeMaps,
    fertilizer_to_water: RangeMaps,
    water_to_light: RangeMaps,
    light_to_temperature: RangeMaps,
    temperature_to_humidity: RangeMaps,
    humidity_to_location: RangeMaps,
}

impl FromStr for Almanac {
    type Err = &'static str;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut parts = text.split("\n\n");

        let seeds_raw: Vec<u64> = parts
            .nth(0)
            .unwrap()
            .trim_start_matches("seeds: ")
            .split(" ")
            .map(|s| s.trim().parse().unwrap())
            .collect();

        let seeds: Vec<(u64, u64)> = seeds_raw
            .chunks_exact(2)
            .map(|chunk| (chunk[0], chunk[1]))
            .collect();

        let seed_to_soil: RangeMaps = parts
            .nth(0)
            .unwrap()
            .splitn(2, "\n")
            .nth(1)
            .unwrap_or("")
            .parse()
            .unwrap();

        let soil_to_fertilizer: RangeMaps = parts
            .nth(0)
            .unwrap()
            .splitn(2, "\n")
            .nth(1)
            .unwrap_or("")
            .parse()
            .unwrap();

        let fertilizer_to_water: RangeMaps = parts
            .nth(0)
            .unwrap()
            .splitn(2, "\n")
            .nth(1)
            .unwrap_or("")
            .parse()
            .unwrap();

        let water_to_light: RangeMaps = parts
            .nth(0)
            .unwrap()
            .splitn(2, "\n")
            .nth(1)
            .unwrap_or("")
            .parse()
            .unwrap();

        let light_to_temperature: RangeMaps = parts
            .nth(0)
            .unwrap()
            .splitn(2, "\n")
            .nth(1)
            .unwrap_or("")
            .parse()
            .unwrap();

        let temperature_to_humidity: RangeMaps = parts
            .nth(0)
            .unwrap()
            .splitn(2, "\n")
            .nth(1)
            .unwrap_or("")
            .parse()
            .unwrap();

        let humidity_to_location: RangeMaps = parts
            .nth(0)
            .unwrap()
            .splitn(2, "\n")
            .nth(1)
            .unwrap_or("")
            .parse()
            .unwrap();

        Ok(Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("./input.prod").expect("input file should exist");

    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("could not read the input file");
    let almanac: Almanac = file_content.parse().unwrap();

    let mut locations: Vec<u64> = Vec::new();
    for seed_range in &almanac.seeds {
        let seeds: Vec<u64> = (seed_range.0..(seed_range.0 + seed_range.1)).collect();
        let seed_locations: Vec<u64> = seeds
            .iter()
            .map(|x| almanac.seed_to_soil.convert(*x))
            .map(|x| almanac.soil_to_fertilizer.convert(x))
            .map(|x| almanac.fertilizer_to_water.convert(x))
            .map(|x| almanac.water_to_light.convert(x))
            .map(|x| almanac.light_to_temperature.convert(x))
            .map(|x| almanac.temperature_to_humidity.convert(x))
            .map(|x| almanac.humidity_to_location.convert(x))
            .collect();
        if let Some(m) = seed_locations.iter().cloned().min() {
            locations.push(m);
        }
    }

    // part1: 35 (dev)
    // part1: 196167384 (prod)

    println!("{:?}", locations.iter().cloned().min().unwrap_or(0));
    // part2: bruteforce attempt takes too much time,
    // I have to optimise this in the future.
    //
    // Ideas to explore:
    //  1. Preprocess the intervals to end up with seed -> locations mapping,
    //      process of finding the min location should be then O(n)
    //  2. Maybe try to inverse the search process,
    //     Instead of trying to compute all possible solutions and then `min` to get the lowest value,
    //     we could try to iteratively check the locations from 1..N and see if there is a corresponding seed.

    Ok(())
}
