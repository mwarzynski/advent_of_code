use std::error::Error;
use std::fmt;
use std::fs::File as FSFile;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
struct MaxHeightPoint {
    x: usize,
    y: usize,
}

struct ViewingDistance {
    up: MaxHeightPoint,
    left: MaxHeightPoint,
    right: MaxHeightPoint,
    down: MaxHeightPoint,
}

impl ViewingDistance {
    fn scenic_score(&self, tree_x: usize, tree_y: usize) -> u32 {
        let up = tree_y as i32 - self.up.y as i32;
        let left = tree_x as i32 - self.left.x as i32;
        let right = self.right.x as i32 - tree_x as i32;
        let down = tree_y as i32 - self.down.y as i32;

        i32::abs(up * left * right * down) as u32
    }
}

#[derive(Debug, Clone, Copy)]
struct Tree {
    max_height_up: Option<u8>,
    max_height_left: Option<u8>,
    max_height_right: Option<u8>,
    max_height_down: Option<u8>,

    height: u8,
}

impl Tree {
    fn new(height: u8) -> Self {
        Self {
            max_height_up: None,
            max_height_left: None,
            max_height_right: None,
            max_height_down: None,
            height,
        }
    }

    fn is_visible(&self) -> bool {
        return vec![
            self.max_height_down,
            self.max_height_up,
            self.max_height_left,
            self.max_height_right,
        ]
        .into_iter()
        .map(|i| match i {
            Some(max_height) => max_height < self.height,
            None => true,
        })
        .any(|i| i);
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct Map {
    trees: Vec<Vec<Tree>>,
}

impl Map {
    fn new() -> Map {
        Map { trees: Vec::new() }
    }

    pub fn add_line(&mut self, line: String) {
        let mut row = Vec::new();
        for c in line.chars() {
            let v = (c as u8) - 48;
            row.push(Tree::new(v));
        }
        self.trees.push(row);
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.trees[0].len(), self.trees.len())
    }

    pub fn count_visible_trees(&self) -> u32 {
        let mut count = 0;
        for y in 0..self.trees.len() {
            for x in 0..self.trees[0].len() {
                if self.get(x, y).is_visible() {
                    count += 1;
                }
            }
        }
        count
    }

    fn get(&self, x: usize, y: usize) -> &Tree {
        &self.trees[y][x % self.trees[0].len()]
    }

    fn get_tree_viewing_distance(&self, x: usize, y: usize) -> ViewingDistance {
        let (width, height) = self.dimensions();

        let tree = self.get(x, y);

        let mut vd = ViewingDistance {
            left: MaxHeightPoint { x, y },
            up: MaxHeightPoint { x, y },
            down: MaxHeightPoint { x, y },
            right: MaxHeightPoint { x, y },
        };

        // left
        for xd in 0..x {
            let x = x - xd - 1;
            let dist_tree = self.get(x, y);
            vd.left.x = x;
            vd.left.y = y;
            if dist_tree.height >= tree.height {
                break;
            }
        }

        // right
        for x in x + 1..width {
            let dist_tree = self.get(x, y);
            vd.right.x = x;
            vd.right.y = y;
            if dist_tree.height >= tree.height {
                break;
            }
        }

        // up
        for yd in 0..y {
            let y = y - yd - 1;
            let dist_tree = self.get(x, y);
            vd.up.x = x;
            vd.up.y = y;
            if dist_tree.height >= tree.height {
                break;
            }
        }

        // down
        for y in y + 1..height {
            let dist_tree = self.get(x, y);
            vd.down.x = x;
            vd.down.y = y;
            if dist_tree.height >= tree.height {
                break;
            }
        }

        vd
    }

    pub fn find_best_scienic_score(&self) -> u32 {
        let mut best_score = 0;
        // iterate over trees and get viewing distance
        for yd in 0..self.trees.len() - 2 {
            for xd in 0..self.trees[0].len() - 2 {
                let x = xd + 1;
                let y = yd + 1;
                let scenic_score = self.get_tree_viewing_distance(x, y).scenic_score(x, y);
                if scenic_score > best_score {
                    best_score = scenic_score;
                }
            }
        }

        best_score
    }

    pub fn eval_trees_visibility(&mut self) {
        let (width, height) = self.dimensions();

        // eval max height up
        for x in 0..width {
            let mut max_height = 0;
            let mut is_on_edge = true;
            for y in 0..height {
                let mut tree = &mut self.trees[y][x % width];
                if !is_on_edge {
                    tree.max_height_up = Some(max_height);
                }
                if tree.height > max_height {
                    max_height = tree.height;
                }
                is_on_edge = false;
            }
        }
        // eval max height down
        for x in 0..width {
            let mut max_height = 0;
            let mut is_on_edge = true;
            for yd in 0..height {
                let y = height - yd - 1;
                let mut tree = &mut self.trees[y][x % width];
                if !is_on_edge {
                    tree.max_height_down = Some(max_height);
                }
                if tree.height > max_height {
                    max_height = tree.height;
                }
                is_on_edge = false;
            }
        }
        // eval max height left
        for y in 0..height {
            let mut max_height = 0;
            let mut is_on_edge = true;
            for x in 0..width {
                let mut tree = &mut self.trees[y][x % width];
                if !is_on_edge {
                    tree.max_height_left = Some(max_height);
                }
                if tree.height > max_height {
                    max_height = tree.height;
                }
                is_on_edge = false;
            }
        }
        // eval max height right
        for y in 0..height {
            let mut max_height = 0;
            let mut is_on_edge = true;
            for xd in 0..width {
                let x = width - xd - 1;
                let mut tree = &mut self.trees[y][x % width];
                if !is_on_edge {
                    tree.max_height_right = Some(max_height);
                }
                if tree.height > max_height {
                    max_height = tree.height;
                }
                is_on_edge = false;
            }
        }
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut map = Map::new();

    let file = FSFile::open("./input.prod").expect("input file should exist");
    for line_wrapped in BufReader::new(file).lines() {
        map.add_line(line_wrapped.unwrap());
    }

    map.eval_trees_visibility();
    println!("visible_trees: {}", map.count_visible_trees());
    println!("best_scenic_score: {}", map.find_best_scienic_score());

    Ok(())
}
