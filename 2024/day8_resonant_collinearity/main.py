from typing import List, Dict, Set

from dataclasses import dataclass


@dataclass
class Point:
    x: int
    y: int

    def __hash__(self) -> int:
        return hash(f"{self.x},{self.y}")


@dataclass
class Map:
    fields: List[str]

    def is_in_bound(self, point: Point) -> bool:
        if point.x < 0 or len(self.fields) <= point.x:
            return False
        if point.y < 0 or len(self.fields[0]) <= point.y:
            return False
        return True


def compute_antinode_locations(antenna_a: Point, antenna_b: Point) -> List[Point]:
    diff_x = antenna_b.x - antenna_a.x
    diff_y = antenna_b.y - antenna_a.y

    point1 = Point(x=antenna_a.x - diff_x, y=antenna_a.y - diff_y)
    point2 = Point(x=antenna_b.x + diff_x, y=antenna_b.y + diff_y)

    return [point1, point2]


def compute_antinode_locations2(map: Map, antenna_a: Point, antenna_b: Point) -> List[Point]:
    diff_x = antenna_b.x - antenna_a.x
    diff_y = antenna_b.y - antenna_a.y

    points = set()

    i = 0
    while True:
        point = Point(x=antenna_a.x + i * diff_x, y=antenna_a.y + i * diff_y)
        if not map.is_in_bound(point):
            break
        points.add(point)
        i += 1

    i = 1
    while True:
        point = Point(x=antenna_a.x - i * diff_x, y=antenna_a.y - i * diff_y)
        if not map.is_in_bound(point):
            break
        points.add(point)
        i += 1

    return list(points)


def parse(file_name: str) -> Map:
    with open(file_name) as f:
        return Map(f.read().strip().split("\n"))


def part1(file_name: str):
    map = parse(file_name)
    all_antennas_locations: Dict[str, Set[Point]] = {}
    for i, row in enumerate(map.fields):
        for j, v in enumerate(row):
            if v != ".":
                collection = all_antennas_locations.get(v, set())
                collection.add(Point(i, j))
                all_antennas_locations[v] = collection

    all_antinode_locations: Set[Point] = set()
    for locations in all_antennas_locations.values():
        locations_list = list(locations)
        for i, location1 in enumerate(locations_list):
            for location2 in locations_list[i + 1 :]:
                for p in compute_antinode_locations(location1, location2):
                    if not map.is_in_bound(p):
                        continue
                    all_antinode_locations.add(p)
                    # print(f"l1={location1}, l2={location2}, antinode={p}")

    print(len(all_antinode_locations))


def part2(file_name: str):
    map = parse(file_name)
    all_antennas_locations: Dict[str, Set[Point]] = {}
    for i, row in enumerate(map.fields):
        for j, v in enumerate(row):
            if v != ".":
                collection = all_antennas_locations.get(v, set())
                collection.add(Point(i, j))
                all_antennas_locations[v] = collection

    all_antinode_locations: Set[Point] = set()
    for locations in all_antennas_locations.values():
        locations_list = list(locations)
        for i, location1 in enumerate(locations_list):
            for location2 in locations_list[i + 1 :]:
                for p in compute_antinode_locations2(map, location1, location2):
                    if not map.is_in_bound(p):
                        continue
                    all_antinode_locations.add(p)
                    # print(f"l1={location1}, l2={location2}, antinode={p}")

    print(len(all_antinode_locations))


def main():
    file_name = "./input.prod"
    part1(file_name)
    part2(file_name)


if __name__ == "__main__":
    main()
