use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs::File as FSFile;
use std::hash::Hasher;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, Clone)]
enum RockShape {
    Minus,
    Cross,
    IL,
    Vertical,
    Block,
}

#[derive(Clone, PartialEq, Debug)]
enum Field {
    Air,
    Rock,
    Unknown,
}

impl ToString for Field {
    fn to_string(&self) -> String {
        match self {
            Field::Air => '.'.to_string(),
            Field::Rock => '#'.to_string(),
            Field::Unknown => '!'.to_string(),
        }
    }
}

#[derive(Debug)]
enum HotGasJet {
    Left,
    Right,
}

impl FromStr for HotGasJet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(HotGasJet::Right),
            "<" => Ok(HotGasJet::Left),
            _ => panic!("invalid hot gas jet character"),
        }
    }
}

#[derive(Debug, Clone)]
struct Rock {
    shape: RockShape,
    x: i32,
    y: i32,
}

impl Rock {
    fn new(x: i32, y: i32, shape: RockShape) -> Self {
        Self { x, y, shape }
    }

    pub fn move_by(&mut self, hot_gas_jet: &HotGasJet) {
        match hot_gas_jet {
            HotGasJet::Left => self.y -= 1,
            HotGasJet::Right => self.y += 1,
        }
    }

    pub fn field_coordinates(&self) -> Vec<(i32, i32)> {
        match self.shape {
            RockShape::Minus => vec![
                (self.x, self.y),
                (self.x, self.y + 1),
                (self.x, self.y + 2),
                (self.x, self.y + 3),
            ],
            RockShape::Cross => vec![
                (self.x - 1, self.y),
                (self.x, self.y + 1),
                (self.x - 1, self.y + 1),
                (self.x - 2, self.y + 1),
                (self.x - 1, self.y + 2),
            ],
            RockShape::IL => vec![
                (self.x, self.y + 2),
                (self.x - 1, self.y + 2),
                (self.x - 2, self.y + 2),
                (self.x - 2, self.y + 1),
                (self.x - 2, self.y),
            ],
            RockShape::Vertical => vec![
                (self.x, self.y),
                (self.x - 1, self.y),
                (self.x - 2, self.y),
                (self.x - 3, self.y),
            ],
            RockShape::Block => vec![
                (self.x, self.y + 1),
                (self.x, self.y),
                (self.x - 1, self.y),
                (self.x - 1, self.y + 1),
            ],
        }
    }
}

struct Game {
    gas_jet_moves: Vec<HotGasJet>,
    map: VecDeque<Vec<Field>>,

    last_rock_shape: usize,
    last_jet_move: usize,
    max_occupied_x: i32,
    map_x_shift: usize,
}

impl Game {
    fn new() -> Self {
        Self {
            gas_jet_moves: Vec::new(),
            map: VecDeque::new(),
            last_rock_shape: 0,
            last_jet_move: 0,
            map_x_shift: 0,
            max_occupied_x: -1,
        }
    }

    pub fn add_gas_jet(&mut self, gas_jet: HotGasJet) {
        self.gas_jet_moves.push(gas_jet);
    }

    pub fn simulate_falling_rock(&mut self) {
        let mut new_rock = self.get_new_rock();
        loop {
            self.rock_push_by_hot_gas(&mut new_rock);
            if !self.rock_move_downward(&mut new_rock) {
                break;
            }
        }
        self.rock_save_in_map(new_rock);

        self.maybe_prune_downwards_memory();
    }

    pub fn tower_height(&self) -> i32 {
        self.max_occupied_x + 1
    }

    #[allow(dead_code)]
    pub fn print(&self, from: usize, to: usize) {
        let n = to - from;
        for id in 0..n {
            let i = to - id - 1;
            print!("{}\t", i);
            if i >= self.map.len() {
                for _ in 0..7 {
                    print!("{}", Field::Air.to_string());
                }
            } else {
                let row = &self.map[i];
                for col in row.iter() {
                    print!("{}", col.to_string());
                }
            }
            println!();
        }
    }

    fn maybe_prune_downwards_memory(&mut self) {
        let max_rows_in_memory = 1_000_000;
        while self.map.len() > max_rows_in_memory {
            self.map.pop_front();
            self.map_x_shift += 1;
        }
    }

    fn rock_push_by_hot_gas(&mut self, rock: &mut Rock) {
        let hot_gas = self.get_hot_gas_jet_move();
        let mut new_rock = rock.clone();
        new_rock.move_by(hot_gas);
        if self.rock_is_invalid(&new_rock) {
            return;
        }
        *rock = new_rock;
    }

    fn get(&self, x: i32, y: i32) -> &Field {
        // Check map bounds first.
        if (y < 0 || y >= 7) || (x < 0) {
            return &Field::Unknown;
        }
        if x < self.map_x_shift as i32 {
            panic!("trying to access a prunned field down");
        }
        let map_x = x as usize - self.map_x_shift;
        if map_x >= self.map.len() {
            return &Field::Air;
        }
        // Attempt to get the field from map vec.
        return self.map.get(map_x).unwrap().get(y as usize).unwrap();
    }

    fn save(&mut self, (x, y): (usize, usize), field: Field) {
        let map_x = x - self.map_x_shift;
        while self.map.len() <= map_x {
            self.map.push_back(vec![Field::Air; 7]);
        }
        match field {
            Field::Rock => {
                if x as i32 > self.max_occupied_x {
                    self.max_occupied_x = x as i32;
                }
            }
            _ => {}
        }
        self.map[map_x][y] = field;
    }

    fn rock_move_downward(&self, rock: &mut Rock) -> bool {
        let mut rock_down = rock.clone();
        rock_down.x -= 1;
        if self.rock_is_invalid(&rock_down) {
            return false;
        }
        *rock = rock_down;
        true
    }

    fn rock_save_in_map(&mut self, rock: Rock) {
        for (x, y) in rock.field_coordinates() {
            self.save((x as usize, y as usize), Field::Rock);
        }
    }

    fn rock_is_invalid(&self, rock: &Rock) -> bool {
        for (x, y) in rock.field_coordinates() {
            let field = self.get(x, y);
            match field {
                Field::Rock => return true,
                Field::Unknown => return true,
                Field::Air => continue,
            }
        }
        false
    }

    fn get_new_rock(&mut self) -> Rock {
        let shape = self.get_new_rock_shape();
        let (x, y) = self.new_rock_coordinates(&shape);
        return Rock::new(x, y, shape);
    }

    fn new_rock_coordinates(&self, shape: &RockShape) -> (i32, i32) {
        let min_x = self.max_occupied_x as i32 + 4;
        let min_y = 2;
        match shape {
            RockShape::Minus => (min_x, min_y),
            RockShape::Cross => (min_x + 2, min_y),
            RockShape::IL => (min_x + 2, min_y),
            RockShape::Vertical => (min_x + 3, min_y),
            RockShape::Block => (min_x + 1, min_y),
        }
    }

    fn get_new_rock_shape(&mut self) -> RockShape {
        self.last_rock_shape = (self.last_rock_shape + 1) % 5;
        match self.last_rock_shape {
            1 => RockShape::Minus,
            2 => RockShape::Cross,
            3 => RockShape::IL,
            4 => RockShape::Vertical,
            0 => RockShape::Block,
            _ => unreachable!("mod, so can't have other values"),
        }
    }

    fn get_hot_gas_jet_move(&mut self) -> &HotGasJet {
        let hot_gas_jet = self.gas_jet_moves.get(self.last_jet_move).unwrap();
        self.last_jet_move = (self.last_jet_move + 1) % self.gas_jet_moves.len();
        return hot_gas_jet;
    }

    fn get_map_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        for y in 0..7 {
            let mut x = self.find_rock_at(y);
            x = self.max_occupied_x as i64 - (x + self.map_x_shift as i64);
            hasher.write_i64(x);
        }
        hasher.finish()
    }

    fn find_rock_at(&self, y: usize) -> i64 {
        for (i, row) in self.map.iter().enumerate().rev() {
            if row[y] == Field::Rock {
                return i as i64;
            }
        }
        return -1;
    }
}

fn main() {
    let mut filename = "./input.dev".to_string();
    if env::args().len() == 2 {
        filename = format!("./input.{}", env::args().nth(1).unwrap()).to_string();
    }

    let mut game = Game::new();

    let file = FSFile::open(filename).expect("input file should exist");
    for line_wrapped in BufReader::new(file).lines() {
        for c in line_wrapped.unwrap().chars() {
            let gas_jet = HotGasJet::from_str(&c.to_string()).unwrap();
            game.add_gas_jet(gas_jet);
        }
    }

    // let blocks_to_add = 1_000_000_000_000 as i64;

    let mut set: HashMap<u64, usize> = HashMap::new();
    let mut max_v: usize = 0;

    for _i in 0..5_000_000 {
        // println!("=== Rock: {} ===", _i + 1);
        game.simulate_falling_rock();

        let h = game.get_map_hash();
        if set.contains_key(&h) {
            let v = set.get(&h).unwrap();
            println!("{}\t{:X} prev={}\theight_current={}\t(max={})", _i, h, v, game.max_occupied_x, max_v);
        } else {
            if _i > max_v {
                max_v = _i;
            }
            set.insert(h, _i);
        }

        if _i % 50455 == 0 {
            println!("{},{}", _i, game.max_occupied_x);
        }
    }

    println!("{}", game.tower_height());
}
