use std::collections::HashMap;
use std::fs::File as FSFile;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Item {
    Air,
    Rock,
    Sand,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

struct Line {
    points: Vec<Point>,
}

impl Line {
    fn new() -> Line {
        Line { points: Vec::new() }
    }

    fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }
}

struct Map {
    map: HashMap<Point, Item>,
    max_rock_x: i32,
    sand_counter: i32,
}

impl Map {
    fn new() -> Map {
        Map {
            map: HashMap::new(),
            max_rock_x: 0,
            sand_counter: 0,
        }
    }

    fn add_rocks(&mut self, line: Line) {
        for i in 1..line.points.len() {
            let mut x1 = line.points[i - 1].x;
            let mut x2 = line.points[i].x;
            if x1 > x2 {
                (x1, x2) = (x2, x1);
            }
            let mut y1 = line.points[i - 1].y;
            let mut y2 = line.points[i].y;
            if y1 > y2 {
                (y1, y2) = (y2, y1);
            }
            for x in x1..x2 + 1 {
                for y in y1..y2 + 1 {
                    if x > self.max_rock_x {
                        self.max_rock_x = x;
                    }
                    self.map.insert(Point::new(x, y), Item::Rock);
                }
            }
        }
    }

    fn get(&self, x: i32, y: i32) -> Item {
        if x == self.max_rock_x + 2 {
            return Item::Rock;
        }
        if let Some(item) = self.map.get(&Point::new(x, y)) {
            return *item;
        }
        return Item::Air;
    }
    fn get_point(&self, point: Point) -> Item {
        return self.get(point.x, point.y);
    }

    // The sand is pouring into the cave from point 500,0.

    fn print(&self, bottom_left: Point, top_right: Point) {
        for x in top_right.x..bottom_left.x + 1 {
            for y in bottom_left.y..top_right.y + 1 {
                match self.get(x, y) {
                    Item::Rock => print!("#"),
                    Item::Sand => print!("o"),
                    Item::Air => print!("."),
                }
            }
            println!();
        }
    }

    fn spawn_sand(&mut self) -> bool {
        let mut sand_point = Point::new(0, 500);
        if self.get_point(sand_point) != Item::Air {
            return false;
        }

        let mut next_sand_point = self.fall_sand(sand_point);

        while next_sand_point != sand_point {
            sand_point = next_sand_point;
            next_sand_point = self.fall_sand(sand_point);
        }

        self.map.insert(sand_point, Item::Sand);
        self.sand_counter += 1;

        return true;
    }

    fn fall_sand(&self, sand_point: Point) -> Point {
        // maybe go down
        let down = Point::new(sand_point.x + 1, sand_point.y);
        if self.get_point(down) == Item::Air {
            return down;
        }
        // maybe go diagonal left
        let diagonal_left = Point::new(sand_point.x + 1, sand_point.y - 1);
        if self.get_point(diagonal_left) == Item::Air {
            return diagonal_left;
        }
        // maybe go diagonal right
        let diagonal_right = Point::new(sand_point.x + 1, sand_point.y + 1);
        if self.get_point(diagonal_right) == Item::Air {
            return diagonal_right;
        }
        sand_point
    }
}

fn main() {
    let mut map = Map::new();

    let file = FSFile::open("./input.prod").expect("input file should exist");
    for line_wrapped in BufReader::new(file).lines() {
        let mut rocks = Line::new();
        for point in line_wrapped.unwrap().split(" -> ") {
            let (y, x) = point.split_once(',').unwrap();
            rocks.add_point(Point::new(x.parse().unwrap(), y.parse().unwrap()));
        }
        map.add_rocks(rocks);
    }

    while map.spawn_sand() {
        // map.print(Point { x: 12, y: 485 }, Point { x: 0, y: 515 });
        // println!()
    }

    println!("sand: {}", map.sand_counter);
}
