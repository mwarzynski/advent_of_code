use regex::Regex;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fs::File as FSFile;
use std::io::{BufReader, Read};
use std::str::FromStr;

type Quantity = i32;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
enum MineralType {
    Geode,
    Obisidian,
    Clay,
    Ore,
}

impl FromStr for MineralType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ore" => Ok(MineralType::Ore),
            "clay" => Ok(MineralType::Clay),
            "obsidian" => Ok(MineralType::Obisidian),
            "geode" => Ok(MineralType::Geode),
            _ => panic!("invalid mineral type: {}", s),
        }
    }
}

type RobotType = MineralType;

type RobotCost = BTreeMap<MineralType, Quantity>;

#[derive(Debug, Hash)]
struct Blueprint {
    id: i32,
    robot_cost: BTreeMap<RobotType, RobotCost>,
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut id = 0;
        let mut robot_cost: BTreeMap<RobotType, RobotCost> = BTreeMap::new();

        let (id_raw, robots) = s.split_once(":").unwrap();

        let re = Regex::new(r"Blueprint (\d+)").unwrap();
        let cap = re.captures(id_raw).unwrap();
        id = (&cap[1]).parse().unwrap();

        for line in robots.split(".") {
            if line.is_empty() {
                continue;
            }
            let re1 = Regex::new(r"Each (ore|clay|obsidian|geode) robot").unwrap();
            let cap1 = re1.captures(line).unwrap();
            let robot_type = MineralType::from_str(&cap1[1]).unwrap();

            let mut mineral_costs: BTreeMap<MineralType, Quantity> = BTreeMap::new();
            let re = Regex::new(r"(\d+) (ore|clay|obsidian|geode)").unwrap();
            for cost in re.captures_iter(line) {
                let mineral = MineralType::from_str(&cost[2]).unwrap();
                mineral_costs.insert(mineral, cost[1].parse().unwrap());
            }

            robot_cost.insert(robot_type, mineral_costs);
        }

        Ok(Self { id, robot_cost })
    }
}

impl Blueprint {
    pub fn quality_level(&self) -> i32 {
        self.max_open_geodes(24) * self.id
    }

    fn max_open_geodes(&self, time_at: i32) -> i32 {
        let mut queue = BinaryHeap::new();
        let mut visited = HashSet::new();
        queue.push(State::new(self));

        let mut max_geodes = 0;

        while !queue.is_empty() {
            let state = queue.pop().unwrap();
            println!(
                "queue_size: {} (time={}) {:?}",
                queue.len(),
                state.time,
                state.resources,
            );

            if state.time == time_at {
                let state_geodes = state.resources_count(&MineralType::Geode);
                if state_geodes > max_geodes {
                    max_geodes = state_geodes;
                }
                continue;
            }

            if visited.contains(&state) {
                continue;
            }
            visited.insert(state.clone());

            for robot_type in vec![
                RobotType::Ore,
                RobotType::Clay,
                RobotType::Obisidian,
                RobotType::Geode,
            ] {
                // maybe produce a new robot
                match state.robot_build_start(robot_type) {
                    Some(mut new_state) => {
                        new_state.tick();
                        queue.push(new_state);
                    }
                    None => {}
                }
            }

            let mut new_state = state.copy();
            new_state.tick();
            queue.push(new_state);
        }

        max_geodes
    }
}

#[derive(Clone, Hash)]
struct State<'a> {
    robots: BTreeMap<RobotType, i32>,
    robots_building: BTreeMap<RobotType, i32>,
    resources: BTreeMap<MineralType, i32>,
    time: i32,
    blueprint: &'a Blueprint,
}

impl Ord for State<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.resources_count(&MineralType::Geode)
            .cmp(&other.resources_count(&MineralType::Geode))
            .then_with(|| {
                self.robots_count(&MineralType::Geode)
                    .cmp(&other.robots_count(&MineralType::Geode))
            })
            .then_with(|| {
                self.resources_count(&MineralType::Obisidian)
                    .cmp(&other.resources_count(&MineralType::Obisidian))
            })
            .then_with(|| {
                self.robots_count(&MineralType::Obisidian)
                    .cmp(&other.robots_count(&MineralType::Obisidian))
            })
            .then_with(|| {
                self.resources_count(&MineralType::Clay)
                    .cmp(&other.resources_count(&MineralType::Clay))
            })
            .then_with(|| {
                self.robots_count(&MineralType::Clay)
                    .cmp(&other.robots_count(&MineralType::Clay))
            })
            .then_with(|| {
                self.resources_count(&MineralType::Ore)
                    .cmp(&other.resources_count(&MineralType::Ore))
            })
            .then_with(|| {
                self.robots_count(&MineralType::Ore)
                    .cmp(&other.robots_count(&MineralType::Ore))
            })
    }
}

impl PartialOrd for State<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.robots == other.robots
            && self.resources == other.resources
            && self.time == other.time
            && self.robots_building == other.robots_building
    }
}

impl Eq for State<'_> {}

impl<'a> State<'a> {
    fn new(blueprint: &'a Blueprint) -> Self {
        let mut robots = BTreeMap::new();
        robots.insert(RobotType::Ore, 1);
        Self {
            robots,
            robots_building: BTreeMap::new(),
            resources: BTreeMap::new(),
            time: 0,
            blueprint,
        }
    }

    pub fn tick(&mut self) {
        self.resources_gather();
        self.robots_build_finish();
        self.time += 1;
    }

    pub fn resources_count(&self, mineral_type: &MineralType) -> i32 {
        match self.resources.get(mineral_type) {
            Some(v) => *v,
            None => 0,
        }
    }

    pub fn robots_count(&self, mineral_type: &MineralType) -> i32 {
        match self.robots.get(mineral_type) {
            Some(v) => *v,
            None => 0,
        }
    }

    pub fn robot_build_start(&self, robot_type: RobotType) -> Option<Self> {
        let mut new_state = self.copy();

        if !self.blueprint.robot_cost.contains_key(&robot_type) {
            return None;
        }
        let robot_cost = self.blueprint.robot_cost.get(&robot_type).unwrap();

        for (mineral_type, required_quantity) in robot_cost {
            if !new_state.resources.contains_key(mineral_type) {
                new_state.resources.insert(mineral_type.clone(), 0);
            }
            let possesed_quantity = match new_state.resources.get_mut(mineral_type) {
                Some(v) => v,
                None => panic!("possesed_quantity must exist"),
            };
            if *possesed_quantity < *required_quantity {
                return None;
            }
            *possesed_quantity -= *required_quantity;
        }

        if new_state.robots_building.get(&robot_type) == None {
            new_state.robots_building.insert(robot_type, 1);
        } else {
            let v = new_state.robots_building.get_mut(&robot_type).unwrap();
            *v += 1;
        }

        Some(new_state)
    }

    fn copy(&self) -> Self {
        Self {
            robots: self.robots.clone(),
            robots_building: self.robots_building.clone(),
            resources: self.resources.clone(),
            time: self.time,
            blueprint: self.blueprint,
        }
    }

    fn resources_gather(&mut self) {
        for (mineral_type, robot_count) in self.robots.iter() {
            if !self.resources.contains_key(mineral_type) {
                self.resources.insert(*mineral_type, 0);
            }
            let resources_count = self.resources.get_mut(mineral_type).unwrap();
            *resources_count += robot_count;
        }
    }

    fn robots_build_finish(&mut self) {
        for (robot_type, newly_build_robot_count) in self.robots_building.iter() {
            if !self.robots.contains_key(robot_type) {
                self.robots.insert(*robot_type, 0);
            }
            let robots_count = self.robots.get_mut(robot_type).unwrap();
            *robots_count += newly_build_robot_count;
        }
        self.robots_building = BTreeMap::new();
    }

    fn value_current(&self) -> i32 {
        self.resources_count(&MineralType::Geode)
    }

    fn value_heuristic(&self) -> i32 {
        let robots_count = match self.robots_building.get(&RobotType::Geode) {
            Some(v) => *v,
            None => 0,
        };
        return robots_count * (24 - self.time);
    }

    fn value_expected(&self) -> i32 {
        self.value_current() + self.value_heuristic()
    }
}

struct Game {
    blueprints: Vec<Blueprint>,
}

impl Game {
    fn new() -> Self {
        Self { blueprints: vec![] }
    }

    fn blueprint_add(&mut self, bp: Blueprint) {
        self.blueprints.push(bp)
    }

    fn sum_quality_levels(&self) -> i32 {
        let mut count = 0;
        for bp in self.blueprints.iter() {
            let ql = bp.quality_level();
            count += ql;
        }
        count
    }
}

fn main() {
    let file = FSFile::open("./input.dev").expect("input file should exist");
    let mut file_content = String::new();
    BufReader::new(file)
        .read_to_string(&mut file_content)
        .expect("input file should be readable");
    let mut game = Game::new();
    for blueprint_raw in file_content.lines() {
        game.blueprint_add(Blueprint::from_str(blueprint_raw.trim_end()).unwrap());
    }
    println!("{}", game.sum_quality_levels());
}

mod tests {
    use super::*;

    #[test]
    fn state() {
        let blueprint = Blueprint::from_str("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.").unwrap();

        let mut state = State::new();
        // Starts with one Ore robot.
        assert_eq!(state.robots[&MineralType::Ore], 1);

        // == Minute 1 ==
        // 1 ore-collecting robot collects 1 ore; you now have 1 ore.
        state.tick();
        assert_eq!(state.robots[&MineralType::Ore], 1);
        assert_eq!(state.resources[&MineralType::Ore], 1);

        match state.robot_build_start(&blueprint, RobotType::Clay) {
            None => {} // expected
            Some(_) => assert!(false),
        }

        // == Minute 2 ==
        // 1 ore-collecting robot collects 1 ore; you now have 2 ore.
        state.tick();
        assert_eq!(state.resources[&MineralType::Ore], 2);
        assert_eq!(state.robots[&MineralType::Ore], 1);

        // == Minute 3 ==
        // Spend 2 ore to start building a clay-collecting robot.
        state = state
            .robot_build_start(&blueprint, RobotType::Clay)
            .unwrap();
        assert_eq!(state.resources[&MineralType::Ore], 0);
        assert_eq!(state.robots[&MineralType::Ore], 1);
        state.tick();
        assert_eq!(state.resources[&MineralType::Ore], 1);
        assert_eq!(state.robots[&MineralType::Ore], 1);
        assert_eq!(state.robots[&MineralType::Clay], 1);

        // == Minute 4 ==
        // 1 ore-collecting robot collects 1 ore; you now have 2 ore.
        // 1 clay-collecting robot collects 1 clay; you now have 1 clay.
        state.tick();
        assert_eq!(state.resources[&MineralType::Ore], 2);
        assert_eq!(state.resources[&MineralType::Clay], 1);
    }

    // #[test]
    // fn quality_level() {
    //     let blueprint = Blueprint::from_str("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.").unwrap();
    //     assert_eq!(blueprint.quality_level(), 9);
    // }

    #[test]
    fn robot_build_start_mutability() {
        let blueprint = Blueprint::from_str("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.").unwrap();
        let mut state = State::new(&blueprint);
        state.tick();
        state.tick();
        state.tick();
        state.tick();
        state.tick();
        state.tick();

        let new_state = state
            .robot_build_start(&blueprint, RobotType::Clay)
            .unwrap();
        assert_ne!(new_state.robots_building.len(), state.robots_building.len());
    }
}
