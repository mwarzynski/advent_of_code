use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File as FSFile;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
struct Name {
    a: char,
    b: char,
}

impl FromStr for Name {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars();
        Ok(Name {
            a: chars.clone().nth(0).unwrap(),
            b: chars.clone().nth(1).unwrap(),
        })
    }
}

#[derive(Debug, Clone)]
struct Valve {
    name: Name,
    flow_rate: i32,
    tunnels: Vec<Name>,
}

impl FromStr for Valve {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (valve, tunnels_raw) = s.split_once("; ").unwrap();
        let (valve_name_raw, flow_rate_raw) = valve.split_once(" has flow rate=").unwrap();
        let (_, name_raw) = valve_name_raw.split_once("Valve ").unwrap();

        let mut tunnels: Vec<Name> = Vec::new();
        if tunnels_raw.contains("to valve ") {
            let name_str = tunnels_raw.split_once("to valve ").unwrap().1.to_string();
            tunnels.push(Name::from_str(&name_str).unwrap());
        } else {
            tunnels = tunnels_raw
                .split_once("lead to valves ")
                .unwrap()
                .1
                .split(", ")
                .map(|s| Name::from_str(&s.to_string()).unwrap())
                .collect();
        }

        Ok(Valve {
            name: Name::from_str(&name_raw.to_string()).unwrap(),
            flow_rate: flow_rate_raw.parse().unwrap(),
            tunnels,
        })
    }
}

#[derive(Debug)]
struct State<'a> {
    name: Name,
    pressure_release_rate: i32,
    pressure_released: i32,
    time: i32,
    valves_open: HashSet<Name>,
    valves: &'a HashMap<Name, Valve>,
}

impl<'a> State<'a> {
    fn next_open(&self) -> Self {
        if self.valves_open.contains(&self.name) {
            panic!("cannot open already opened valve");
        }
        let valve = self.valves.get(&self.name).unwrap();
        let pressure_released = self.pressure_released + self.pressure_release_rate;
        let pressure_release_rate = self.pressure_release_rate + valve.flow_rate;
        let mut valves_open = self.valves_open.clone();
        valves_open.insert(self.name.clone());
        State {
            name: self.name,
            pressure_release_rate,
            pressure_released,
            time: self.time + 1,
            valves_open,
            valves: self.valves,
        }
    }

    fn next_move(&self, valve: Valve) -> Self {
        let pressure_released = self.pressure_released + self.pressure_release_rate;
        let pressure_release_rate = self.pressure_release_rate;
        let valves_open = self.valves_open.clone();
        State {
            name: valve.name,
            pressure_release_rate,
            pressure_released,
            time: self.time + 1,
            valves_open,
            valves: self.valves,
        }
    }
}

impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for State<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.time.eq(&other.time).then_some(|| self.name.eq(&other.name))
    }
}

impl<'a> Eq for State<'a> {}

fn release_the_most_pressure(valves: Vec<Valve>) -> i32 {
    let time_max = 30;
    let valves_map: HashMap<Name, Valve> = valves.into_iter().map(|v| (v.name, v)).collect();

    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State {
        name: Name::from_str("AA").unwrap(),
        pressure_release_rate: 0,
        pressure_released: 0,
        time: 0,
        valves_open: HashSet::new(),
        valves: &valves_map,
    });

    while !queue.is_empty() {
        let state = queue.pop().unwrap();
        let valve = valves_map.get(&state.name).unwrap();

        if state.time >= time_max {
            return state.pressure_released;
        }

        // try to move to another valve
        for neighbour in valve.tunnels.iter() {
            let neighbour_valve = valves_map.get(&neighbour).unwrap();
            queue.push(state.next_move(neighbour_valve.clone()));
        }

        // try to open current valve
        if !state.valves_open.contains(&valve.name) {
            if valve.flow_rate > 0 {
                queue.push(state.next_open());
            }
        }
    }

    0
}

fn main() {
    let mut valves = Vec::new();
    let file = FSFile::open("./input.dev").expect("input file should exist");
    for line_wrapped in BufReader::new(file).lines() {
        let valve = Valve::from_str(&line_wrapped.unwrap()).unwrap();
        valves.push(valve);
    }

    let pressure = release_the_most_pressure(valves);

    println!("most pressure we can release: {:?}", pressure,);
}

mod tests {
    use super::*;

    #[test]
    fn test_find_most_pressure_we_can_release_dev() {
        let mut network_of_pipes = NetworkOfPipes::new();
        let file = FSFile::open("./input.dev").expect("input file should exist");
        for line_wrapped in BufReader::new(file).lines() {
            let valve = Valve::from_str(&line_wrapped.unwrap()).unwrap();
            network_of_pipes.add_valve(valve);
        }
        assert_eq!(network_of_pipes.find_most_pressure_we_can_release(30), 1651);
    }

    #[test]
    fn test_find_most_pressure_we_can_release_prod() {
        let mut network_of_pipes = NetworkOfPipes::new();
        let file = FSFile::open("./input.prod").expect("input file should exist");
        for line_wrapped in BufReader::new(file).lines() {
            let valve = Valve::from_str(&line_wrapped.unwrap()).unwrap();
            network_of_pipes.add_valve(valve);
        }
        assert_eq!(network_of_pipes.find_most_pressure_we_can_release(30), 1653);
    }
}
