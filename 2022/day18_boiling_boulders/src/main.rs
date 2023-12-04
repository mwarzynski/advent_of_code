use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File as FSFile;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Ord, PartialOrd)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn new(x: i32, y: i32, z: i32) -> Self {
        return Self { x, y, z };
    }

    fn next(&self, x: i32, y: i32, z: i32) -> Self {
        Self::new(self.x + x, self.y + y, self.z + z)
    }

    fn nexts(&self) -> Vec<Self> {
        let mut ns = vec![];
        if self.x >= 0 {
            ns.push(self.next(-1, 0, 0));
        }
        if self.y >= 0 {
            ns.push(self.next(0, -1, 0));
        }
        if self.z >= 0 {
            ns.push(self.next(0, 0, -1));
        }
        ns.push(self.next(1, 0, 0));
        ns.push(self.next(0, 1, 0));
        ns.push(self.next(0, 0, 1));
        ns
    }

    fn nexts_ordered(&self) -> Vec<Self> {
        vec![self.next(1, 0, 0), self.next(0, 1, 0), self.next(0, 0, 1)]
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

#[derive(Debug)]
struct Lava {
    cubes: HashSet<Cube>,
    cube_is_outside_air: HashMap<Cube, bool>,
}

impl Lava {
    fn new() -> Self {
        return Self {
            cubes: HashSet::new(),
            cube_is_outside_air: HashMap::new(),
        };
    }

    pub fn add_cube(&mut self, cube: Cube) {
        self.cubes.insert(cube);
    }

    pub fn surface_area(&mut self) -> u32 {
        let mut count: u32 = self.cubes.len() as u32 * 6;

        let mut cl: Vec<&Cube> = self.cubes.iter().collect();
        cl.sort();

        for cube in cl {
            for cube_next in cube.nexts_ordered() {
                if self.cubes.contains(&cube_next) {
                    count -= 2;
                }
            }
        }

        count
    }

    pub fn surface_external_area(&mut self) -> u32 {
        let mut count: u32 = self.cubes.len() as u32 * 6;

        let mut cl: Vec<Cube> = self.cubes.iter().map(|c| c.clone()).collect();
        cl.sort();

        for cube in cl {
            for cube_next in cube.nexts() {
                if self.cubes.contains(&cube_next) {
                    count -= 1;
                    continue;
                }
                if self.cube_is_internal_air(&cube_next) {
                    count -= 1;
                }
            }
        }

        count
    }

    fn cube_is_internal_air(&mut self, cube: &Cube) -> bool {
        let cube_external = Cube::new(-1, -1, -1);
        let mut visited = HashSet::new();
        return !self.cube_outside_air_reachable(cube, &cube_external, &mut visited);
    }

    fn cube_outside_air_reachable(
        &mut self,
        cube1: &Cube,
        cube2: &Cube,
        visited: &mut HashSet<Cube>,
    ) -> bool {
        if self.cubes.contains(cube1) {
            return false;
        }

        // dfs
        for cube in cube1.nexts() {
            if cube == *cube2 {
                return true;
            }
            if self.cubes.contains(&cube) {
                // is lava, continue
                continue;
            }
            if self.cube_is_outside_air.contains_key(&cube) {
                let is_outside = self.cube_is_outside_air.get(&cube).unwrap();
                if *is_outside {
                    return true;
                } else {
                    continue;
                }
            }
            if visited.contains(&cube) {
                // already computed, continue
                continue;
            }
            visited.insert(cube.clone());

            if self.cube_outside_air_reachable(&cube, cube2, visited) {
                self.cube_is_outside_air.insert(cube.clone(), true);
                return true;
            }
        }

        self.cube_is_outside_air.insert(cube1.clone(), false);
        false
    }
}

fn main() {
    let mut lava = Lava::new();

    let file = FSFile::open("./input.prod").expect("input file should exist");
    for line_wrapped in BufReader::new(file).lines() {
        let line = line_wrapped.unwrap();
        let vs: Vec<i32> = line.split(",").map(|i| i.parse().unwrap()).collect();
        lava.add_cube(Cube::new(vs[0], vs[1], vs[2]));
    }

    println!("{}", lava.surface_area());
    println!("{}", lava.surface_external_area());
}
