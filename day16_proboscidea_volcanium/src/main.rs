use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File as FSFile;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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

#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(PartialEq, Eq)]
struct Node<'a> {
    time: u32,
<<<<<<< HEAD
=======
    time_finish: u32,
>>>>>>> 505b06e (aoc16: wip)
    total_pressure_released: i32,
    valve: &'a Valve,
    current_pressure_release: i32,
    open_valves: HashSet<Name>,
}

impl<'a> Node<'a> {
<<<<<<< HEAD
    fn expected_pressure_released(&self, time_at: u32) -> i32 {
        let time_left = 30 - time_at;
=======
    fn expected_pressure_released(&self) -> i32 {
        let time_left = self.time_finish as i32 - self.time as i32;
        if time_left < 0 {
            return self.total_pressure_released;
        }
>>>>>>> 505b06e (aoc16: wip)
        let mut pressure_released = self.total_pressure_released;
        pressure_released += self.current_pressure_release * time_left as i32;
        pressure_released
    }
}

impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
<<<<<<< HEAD
        self.expected_pressure_released(30)
            .cmp(&other.expected_pressure_released(30))
=======
        self.expected_pressure_released()
            .cmp(&other.expected_pressure_released())
>>>>>>> 505b06e (aoc16: wip)
    }
}

// `PartialOrd` needs to be implemented as well.
impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct NetworkOfPipes {
    valves: HashMap<Name, Valve>,
}

impl<'a> NetworkOfPipes {
    pub fn new() -> Self {
        NetworkOfPipes {
            valves: HashMap::new(),
        }
    }

    pub fn add_valve(&mut self, valve: Valve) {
        self.valves.insert(valve.name, valve.clone());
    }

    pub fn find_most_pressure_we_can_release(&'a self, time_at: u32) -> i32 {
        let mut queue: BinaryHeap<Node> = BinaryHeap::new();

        queue.push(Node {
            valve: &self
                .valves
                .get(&Name::from_str("AA").unwrap())
                .expect("AA valve must exist, or else aoc lied to us"),
            time: 1,
<<<<<<< HEAD
=======
            time_finish: time_at,
>>>>>>> 505b06e (aoc16: wip)
            current_pressure_release: 0,
            total_pressure_released: 0,
            open_valves: HashSet::new(),
        });
        let mut time_valve_best_pressure: HashMap<u32, HashMap<Name, (i32, i32)>> = HashMap::new();

        let mut max_pressure_release = 0;
        while !queue.is_empty() {
            let node = queue.pop().unwrap();

<<<<<<< HEAD
            // check if already visited
            let value_best_pressure: &mut HashMap<Name, (i32, i32)>;
            if time_valve_best_pressure.contains_key(&node.time) {
                value_best_pressure = time_valve_best_pressure.get_mut(&node.time).unwrap();
            } else {
                time_valve_best_pressure.insert(node.time, HashMap::new());
                value_best_pressure = time_valve_best_pressure.get_mut(&node.time).unwrap();
            }
=======
            if node.time == time_at + 1 {
                if node.total_pressure_released > max_pressure_release {
                    max_pressure_release = node.total_pressure_released;
                }
                continue;
            }

            // check if already visited
            let value_best_pressure: &mut HashMap<Name, (i32, i32)>;
            if !time_valve_best_pressure.contains_key(&node.time) {
                time_valve_best_pressure.insert(node.time, HashMap::new());
            }
            value_best_pressure = time_valve_best_pressure.get_mut(&node.time).unwrap();
>>>>>>> 505b06e (aoc16: wip)
            if value_best_pressure.contains_key(&node.valve.name) {
                let (best_total_pressure_released, best_rate_pressure) =
                    value_best_pressure.get(&node.valve.name).unwrap();
                if node.total_pressure_released <= *best_total_pressure_released
                    && node.current_pressure_release <= *best_rate_pressure
                {
                    continue;
                }
            }
            value_best_pressure.insert(
                node.valve.name,
                (node.total_pressure_released, node.current_pressure_release),
            );

            // println!(
            //     "queue len: {} {}{} (time={}m)\t(pressure_released={})\t(pressure_release_rate={})",
            //     queue.len(),
<<<<<<< HEAD
            //     node.valve.name[0],
            //     node.valve.name[1],
=======
            //     node.valve.name.a,
            //     node.valve.name.b,
>>>>>>> 505b06e (aoc16: wip)
            //     node.time,
            //     node.total_pressure_released,
            //     node.current_pressure_release,
            // );

<<<<<<< HEAD
            if node.time == time_at {
                let total_pressure_released =
                    node.total_pressure_released + node.current_pressure_release;
                if total_pressure_released > max_pressure_release {
                    max_pressure_release = total_pressure_released;
                }
                continue;
            }

=======
>>>>>>> 505b06e (aoc16: wip)
            // option 1: open a valve
            if !node.open_valves.contains(&node.valve.name) {
                let mut new_open_valves = node.open_valves.clone();
                new_open_valves.insert(node.valve.name);
                let total_pressure_released =
                    node.total_pressure_released + node.current_pressure_release;
                let current_pressure_release = node.current_pressure_release + node.valve.flow_rate;
<<<<<<< HEAD

                queue.push(Node {
                    valve: node.valve,
                    time: node.time + 1,
=======
                if node.valve.name.a == 'D' && node.valve.name.b == 'T' {
                    println!("OPENING DT at {}", node.time);
                }
                queue.push(Node {
                    valve: node.valve,
                    time: node.time + 1,
                    time_finish: time_at,
>>>>>>> 505b06e (aoc16: wip)
                    current_pressure_release,
                    total_pressure_released,
                    open_valves: new_open_valves,
                })
            }
<<<<<<< HEAD

=======
>>>>>>> 505b06e (aoc16: wip)
            // option 2. go through a tunnel
            let unopened_valves_exist = node.open_valves.len() < self.valves.len();
            if unopened_valves_exist {
                for tunnel in node.valve.tunnels.iter() {
                    let valve = self.valves.get(tunnel).unwrap();
                    let total_pressure_released =
                        node.total_pressure_released + node.current_pressure_release;
<<<<<<< HEAD

                    let time = node.time + 1;

                    queue.push(Node {
                        valve,
                        time,
=======
                    let time = node.time + 1;
                    queue.push(Node {
                        valve,
                        time,
                        time_finish: time_at,
>>>>>>> 505b06e (aoc16: wip)
                        current_pressure_release: node.current_pressure_release,
                        total_pressure_released,
                        open_valves: node.open_valves.clone(),
                    })
                }
<<<<<<< HEAD
            } else {
                // option 3: stay in place and wait for the next time step
                queue.push(Node {
                    valve: node.valve,
                    time: node.time + 1,
                    current_pressure_release: node.current_pressure_release,
                    total_pressure_released: node.total_pressure_released
                        + node.current_pressure_release,
                    open_valves: node.open_valves.clone(),
                })
            }
=======
            }
            // option 3: stay in place and wait for the next time step
            queue.push(Node {
                valve: node.valve,
                time: node.time + 1,
                time_finish: time_at,
                current_pressure_release: node.current_pressure_release,
                total_pressure_released: node.total_pressure_released
                    + node.current_pressure_release,
                open_valves: node.open_valves.clone(),
            })
>>>>>>> 505b06e (aoc16: wip)
        }

        max_pressure_release
    }
}

fn main() {
    let mut network_of_pipes = NetworkOfPipes::new();

    let file = FSFile::open("./input.prod").expect("input file should exist");
    for line_wrapped in BufReader::new(file).lines() {
        let valve = Valve::from_str(&line_wrapped.unwrap()).unwrap();
        println!("{:?}", valve);
        network_of_pipes.add_valve(valve);
    }

    println!(
        "most pressure we can release: {:?}",
        network_of_pipes.find_most_pressure_we_can_release(30)
    );
<<<<<<< HEAD
    // your answer is too low: 1621
=======
>>>>>>> 505b06e (aoc16: wip)
}

mod tests {
    use super::*;

<<<<<<< HEAD
    #[test]
    fn test_find_most_pressure_we_can_release_dev() {
        let mut network_of_pipes = NetworkOfPipes::new();
        let file = FSFile::open("./input.dev").expect("input file should exist");
=======
    // #[test]
    // fn test_find_most_pressure_we_can_release_dev() {
    //     let mut network_of_pipes = NetworkOfPipes::new();
    //     let file = FSFile::open("./input.dev").expect("input file should exist");
    //     for line_wrapped in BufReader::new(file).lines() {
    //         let valve = Valve::from_str(&line_wrapped.unwrap()).unwrap();
    //         network_of_pipes.add_valve(valve);
    //     }
    //     assert_eq!(network_of_pipes.find_most_pressure_we_can_release(30), 1651);
    // }

    #[test]
    fn test_find_most_pressure_we_can_release_prod() {
        let mut network_of_pipes = NetworkOfPipes::new();
        let file = FSFile::open("./input.prod").expect("input file should exist");
>>>>>>> 505b06e (aoc16: wip)
        for line_wrapped in BufReader::new(file).lines() {
            let valve = Valve::from_str(&line_wrapped.unwrap()).unwrap();
            network_of_pipes.add_valve(valve);
        }
<<<<<<< HEAD
        assert_eq!(network_of_pipes.find_most_pressure_we_can_release(30), 1651);
    }

    #[test]
    fn test_find_most_pressure_we_can_release_1() {
        let mut network_of_pipes = NetworkOfPipes::new();
        let a = Name::from_str("AA").unwrap();
        let b = Name::from_str("BB").unwrap();
        let c = Name::from_str("CC").unwrap();
        let d = Name::from_str("DD").unwrap();
        network_of_pipes.add_valve(Valve {
            name: a,
            flow_rate: 0,
            tunnels: vec![b],
        });
        network_of_pipes.add_valve(Valve {
            name: b,
            flow_rate: 1,
            tunnels: vec![c],
        });
        network_of_pipes.add_valve(Valve {
            name: c,
            flow_rate: 10,
            tunnels: vec![d],
        });
        network_of_pipes.add_valve(Valve {
            name: d,
            flow_rate: 0,
            tunnels: vec![a, b, c],
        });
        assert_eq!(network_of_pipes.find_most_pressure_we_can_release(2), 0);
        assert_eq!(network_of_pipes.find_most_pressure_we_can_release(3), 1);
        assert_eq!(network_of_pipes.find_most_pressure_we_can_release(4), 10);
        assert_eq!(network_of_pipes.find_most_pressure_we_can_release(5), 20);
    }

    #[test]
    fn test_find_most_pressure_we_can_release_2() {
        let mut network_of_pipes = NetworkOfPipes::new();
        let a = Name::from_str("AA").unwrap();
        let b = Name::from_str("BB").unwrap();
        let c = Name::from_str("CC").unwrap();
        let d = Name::from_str("DD").unwrap();
        network_of_pipes.add_valve(Valve {
            name: a,
            flow_rate: 100,
            tunnels: vec![b],
        });
        network_of_pipes.add_valve(Valve {
            name: b,
            flow_rate: 1,
            tunnels: vec![c],
        });
        network_of_pipes.add_valve(Valve {
            name: c,
            flow_rate: 10,
            tunnels: vec![d],
        });
        network_of_pipes.add_valve(Valve {
            name: d,
            flow_rate: 0,
            tunnels: vec![a, b, c],
        });
        assert_eq!(network_of_pipes.find_most_pressure_we_can_release(2), 100);
        assert_eq!(network_of_pipes.find_most_pressure_we_can_release(3), 200);
        assert_eq!(network_of_pipes.find_most_pressure_we_can_release(4), 301);
=======
        assert_eq!(network_of_pipes.find_most_pressure_we_can_release(30), 1653);
>>>>>>> 505b06e (aoc16: wip)
    }
}
