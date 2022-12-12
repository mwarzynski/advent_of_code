use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fs::File as FSFile;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct PositionWithSteps {
    steps: u32,
    position: Position,
}

#[derive(Debug)]
struct Hill {
    grid: Vec<Vec<char>>,
    start: Position,
    destination: Position,
}

impl Hill {
    pub fn new() -> Hill {
        Hill {
            grid: Vec::new(),
            start: Position { x: 0, y: 0 },
            destination: Position { x: 0, y: 0 },
        }
    }

    pub fn add_grid_line(&mut self, line: String) {
        let mut grid_line = Vec::new();
        for (i, c) in line.chars().enumerate() {
            if c == 'S' {
                self.start = Position {
                    x: i as u32,
                    y: self.grid.len() as u32,
                };
            }
            if c == 'E' {
                self.destination = Position {
                    x: i as u32,
                    y: self.grid.len() as u32,
                };
            }
            grid_line.push(c);
        }
        self.grid.push(grid_line);
    }

    pub fn climb(&self) -> u32 {
        // hashset of (position, steps)
        let mut visited: HashSet<Position> = HashSet::new();
        // queue for next steps to check
        let mut queue: BinaryHeap<Reverse<PositionWithSteps>> = BinaryHeap::new();

        queue.push(Reverse(PositionWithSteps {
            position: self.start,
            steps: 0,
        }));
        let mut end_reached = false;
        let mut climb_steps = 0;
        while !end_reached && !queue.is_empty() {
            let Reverse(PositionWithSteps { position, steps }) = queue.pop().unwrap();
            if visited.contains(&position) {
                continue;
            }
            let steps = self
                .possible_steps_from(position)
                .into_iter()
                .filter(|pos| self.can_do_step_up(position, *pos))
                .map(|pos| PositionWithSteps {
                    position: pos,
                    steps: steps + 1,
                })
                .collect::<Vec<PositionWithSteps>>();
            for step in steps {
                if step.position == self.destination {
                    climb_steps = step.steps;
                    end_reached = true;
                    break;
                }
                queue.push(Reverse(step));
            }
            visited.insert(position);
        }

        climb_steps
    }

    pub fn find_hiking_trail(&self) -> u32 {
        // hashset of (position, steps)
        let mut visited: HashSet<Position> = HashSet::new();
        // queue for next steps to check
        let mut queue: BinaryHeap<Reverse<PositionWithSteps>> = BinaryHeap::new();

        queue.push(Reverse(PositionWithSteps {
            position: self.destination,
            steps: 0,
        }));
        let mut end_reached = false;
        let mut trail_steps = 0;
        while !end_reached && !queue.is_empty() {
            let Reverse(PositionWithSteps { position, steps }) = queue.pop().unwrap();
            if visited.contains(&position) {
                continue;
            }
            let steps = self
                .possible_steps_from(position)
                .into_iter()
                .filter(|pos| self.can_do_step_down(position, *pos))
                .map(|pos| PositionWithSteps {
                    position: pos,
                    steps: steps + 1,
                })
                .collect::<Vec<PositionWithSteps>>();
            for step in steps {
                let c = self.get_position(step.position);
                if c == 'a' || c == 'S' {
                    trail_steps = step.steps;
                    end_reached = true;
                    break;
                }
                queue.push(Reverse(step));
            }
            visited.insert(position);
        }

        trail_steps
    }

    fn can_do_step_up(&self, a: Position, b: Position) -> bool {
        let mut a_char = self.get_position(a);
        let mut b_char = self.get_position(b);
        if a_char == 'S' {
            a_char = 'a';
        }
        if b_char == 'S' {
            b_char = 'a';
        }
        if b_char == 'E' {
            b_char = 'z';
        }
        b_char as i32 <= a_char as i32 + 1
    }

    fn can_do_step_down(&self, a: Position, b: Position) -> bool {
        let mut a_char = self.get_position(a);
        let mut b_char = self.get_position(b);
        if a_char == 'S' {
            a_char = 'a';
        }
        if b_char == 'S' {
            b_char = 'a';
        }
        if a_char == 'E' {
            a_char = 'z';
        }
        if b_char == 'E' {
            b_char = 'z';
        }
        a_char as i32 <= (b_char as i32) + 1
    }

    fn possible_steps_from(&self, pos: Position) -> Vec<Position> {
        let mut possible_steps = Vec::new();
        if pos.x > 0 {
            possible_steps.push(Position {
                x: pos.x - 1,
                y: pos.y,
            });
        }
        if pos.x < self.grid[0].len() as u32 - 1 {
            possible_steps.push(Position {
                x: pos.x + 1,
                y: pos.y,
            });
        }
        if pos.y > 0 {
            possible_steps.push(Position {
                x: pos.x,
                y: pos.y - 1,
            });
        }
        if pos.y < self.grid.len() as u32 - 1 {
            possible_steps.push(Position {
                x: pos.x,
                y: pos.y + 1,
            });
        }
        possible_steps
    }

    fn get_position(&self, pos: Position) -> char {
        self.grid[pos.y as usize][pos.x as usize]
    }
}

fn main() {
    let mut hill = Hill::new();

    let file = FSFile::open("./input.prod").expect("input file should exist");
    for line_wrapped in BufReader::new(file).lines() {
        hill.add_grid_line(line_wrapped.unwrap());
    }

    println!("steps: {}", hill.climb());
    println!("steps: {}", hill.find_hiking_trail())
}
