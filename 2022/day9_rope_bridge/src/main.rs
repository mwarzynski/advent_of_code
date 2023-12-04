use std::collections::HashSet;
use std::error::Error;
use std::fs::File as FSFile;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
struct ParseError;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
struct Move {
    direction: Direction,
    length: i32,
}

impl FromStr for Move {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (direction, length) = line.split_once(" ").unwrap();
        match direction {
            "U" => Ok(Self {
                direction: Direction::Up,
                length: length.parse().unwrap(),
            }),
            "D" => Ok(Self {
                direction: Direction::Down,
                length: length.parse().unwrap(),
            }),
            "L" => Ok(Self {
                direction: Direction::Left,
                length: length.parse().unwrap(),
            }),
            "R" => Ok(Self {
                direction: Direction::Right,
                length: length.parse().unwrap(),
            }),
            _ => Err(ParseError),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Field {
    x: i32,
    y: i32,
}

impl Field {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct Rope {
    parts: Vec<Field>,
}

impl Rope {
    fn new(size: usize) -> Self {
        Self {
            parts: vec![Field::new(0, 0); size],
        }
    }

    pub fn drag_towards(&mut self, direction: Direction) {
        let mut rope_head = self.parts.get_mut(0).unwrap();
        match direction {
            Direction::Up => rope_head.y += 1,
            Direction::Down => rope_head.y -= 1,
            Direction::Left => rope_head.x -= 1,
            Direction::Right => rope_head.x += 1,
        }

        for i in 1..self.parts.len() {
            let head = *self.parts.get(i - 1).unwrap();
            let tail = self.parts.get_mut(i).unwrap();

            if !Self::field_needs_to_move(&head, tail) {
                continue;
            }

            if head.x == tail.x {
                // move up or down
                if head.y > tail.y {
                    tail.y += 1;
                }
                if head.y < tail.y {
                    tail.y -= 1;
                }
            } else if head.y == tail.y {
                // move left or right
                if head.x > tail.x {
                    tail.x += 1;
                }
                if head.x < tail.x {
                    tail.x -= 1;
                }
            } else {
                // move diagonal
                if head.x > tail.x {
                    tail.x += 1;
                } else {
                    tail.x -= 1;
                }
                if head.y > tail.y {
                    tail.y += 1;
                } else {
                    tail.y -= 1;
                }
            }
        }
    }

    pub fn get(&self) -> &Vec<Field> {
        &self.parts
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for i in 0..self.parts.len() {
            print!("({},{}) ", self.parts[i].x, self.parts[i].y);
        }
        println!()
    }

    fn field_needs_to_move(a: &Field, b: &Field) -> bool {
        i32::abs(a.x - b.x) > 1 || i32::abs(a.y - b.y) > 1
    }
}

#[allow(dead_code)]
struct Map {
    height_min: i32,
    width_min: i32,
    height_max: i32,
    width_max: i32,
}

#[allow(dead_code)]
impl Map {
    fn new() -> Self {
        Self {
            height_min: -5,
            height_max: 15,
            width_min: -11,
            width_max: 15,
        }
    }

    fn print(&self, parts: &Vec<Field>) {
        for y in (self.height_min..self.height_max).rev() {
            for x in self.width_min..self.width_max {
                let mut found = false;
                for i in 0..parts.len() {
                    let part = parts[i];
                    if x == part.x && y == part.y {
                        if i == 0 {
                            print!("H");
                        } else {
                            print!("{}", i);
                        }
                        found = true;
                        break;
                    }
                }
                if !found {
                    print!(".");
                }
            }
            println!();
        }
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut rope = Rope::new(10);
    let mut fields_visited_by_tail: HashSet<Field> = HashSet::new();
    // let map = Map::new();

    let file = FSFile::open("./input.prod").expect("input file should exist");
    for line_wrapped in BufReader::new(file).lines() {
        let m = Move::from_str(&line_wrapped.unwrap()).unwrap();

        for _ in 0..m.length {
            // map.print(&rope.parts);
            // rope.print();
            // println!();

            rope.drag_towards(m.direction);

            fields_visited_by_tail.insert(*rope.get().last().unwrap());
        }
    }
    // let v = fields_visited_by_tail.iter().map(|i| i.clone()).collect();
    // map.print(&v);

    println!("positions_tail_visited: {}", fields_visited_by_tail.len()); // 6367

    Ok(())
}
