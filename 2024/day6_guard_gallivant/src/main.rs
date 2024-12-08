use std::collections::HashSet;
use std::io::{self, Read};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    /// Rotates the direction clockwise.
    fn turn(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    /// Returns all possible directions.
    fn all() -> [Self; 4] {
        [Self::Up, Self::Right, Self::Down, Self::Left]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    /// Creates a new point.
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Moves the point in the given direction.
    fn go(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Down => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Left => Self {
                x: self.x,
                y: self.y - 1,
            },
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct PointDirection {
    point: Point,
    direction: Direction,
}

impl PointDirection {
    /// Creates a new PointDirection.
    fn new(point: Point, direction: Direction) -> Self {
        Self { point, direction }
    }
}

#[derive(Debug, Clone)]
struct Map {
    fields: Vec<Vec<char>>,
}

impl Map {
    /// Creates a new Map by reading from standard input.
    fn new_from_stdin() -> io::Result<Self> {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        let trimmed = input.trim();
        let fields = trimmed.lines().map(|line| line.chars().collect()).collect();
        Ok(Self { fields })
    }

    /// Finds all points containing the specified character.
    fn find_field(&self, c: char) -> Vec<Point> {
        self.fields
            .iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.iter().enumerate().filter_map(move |(y, &ch)| {
                    if ch == c {
                        Some(Point::new(x as i32, y as i32))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    /// Retrieves the character at the given point, if within bounds.
    fn get_field(&self, point: Point) -> Option<char> {
        self.fields
            .get(point.x as usize)?
            .get(point.y as usize)
            .copied()
    }

    /// Sets the character at the given point, if within bounds.
    fn set_field(&mut self, point: Point, value: char) -> Option<()> {
        if let Some(row) = self.fields.get_mut(point.x as usize) {
            if row.get_mut(point.y as usize).is_some() {
                row[point.y as usize] = value;
                return Some(());
            }
        }
        None
    }
}

enum GuardPathingResult {
    Cycle,
    OutOfMap,
}

fn guard_pathing(
    map: &Map,
    point_start: Point,
    direction_strt: Direction,
    previously_visited: &HashSet<PointDirection>,
) -> GuardPathingResult {
    let mut point_current = point_start;
    let mut direction_current = direction_strt;
    let mut visited = HashSet::new();

    loop {
        let pd = PointDirection::new(point_current, direction_current);
        if !visited.insert(pd) || previously_visited.contains(&pd) {
            return GuardPathingResult::Cycle;
        }

        let point_next = point_current.go(direction_current);

        match map.get_field(point_next) {
            Some('#') => {
                direction_current = direction_current.turn();
            }
            Some('.') | Some('^') => {
                point_current = point_next;
            }
            Some(other) => {
                panic!("Encountered unknown character: {}", other);
            }
            None => return GuardPathingResult::OutOfMap,
        }
    }
}

fn part2(map: &Map, start: Point) -> usize {
    let mut point_current = start;
    let mut direction_current = Direction::Up;
    let mut visited = HashSet::new();
    let mut obstruction_points = HashSet::new();

    let map = map.clone(); // Clone once to allow modifications.

    loop {
        let next_point = point_current.go(direction_current);

        if let Some('.') = map.get_field(next_point) {
            let already_visited = Direction::all()
                .iter()
                .any(|&d| visited.contains(&PointDirection::new(next_point, d)));

            if !already_visited {
                let mut modified_map = map.clone();
                modified_map.set_field(next_point, '#');

                if let GuardPathingResult::Cycle =
                    guard_pathing(&modified_map, point_current, direction_current, &visited)
                {
                    obstruction_points.insert(next_point);
                }
            }
        }

        match map.get_field(next_point) {
            Some('#') => {
                direction_current = direction_current.turn();
            }
            Some('.') | Some('^') => {
                visited.insert(PointDirection::new(point_current, direction_current));
                point_current = next_point;
            }
            Some(other) => {
                panic!("Encountered unknown character: {}", other);
            }
            None => break,
        }
    }

    obstruction_points.len()
}

fn main() -> io::Result<()> {
    let map = Map::new_from_stdin()?;
    let start_positions = map.find_field('^');

    if start_positions.len() != 1 {
        eprintln!(
            "Expected exactly one starting position '^', found {}",
            start_positions.len()
        );
        std::process::exit(1);
    }

    let result = part2(&map, start_positions[0]);
    println!("{}", result);

    Ok(())
}

