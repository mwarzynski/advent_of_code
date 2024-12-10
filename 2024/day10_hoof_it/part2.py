from typing import Dict, List, Self, Set
from dataclasses import dataclass
import queue


@dataclass
class Point:

    x: int
    y: int

    def nexts(self) -> List[Self]:
        return [
            self.__class__(self.x + 1, self.y),
            self.__class__(self.x - 1, self.y),
            self.__class__(self.x, self.y + 1),
            self.__class__(self.x, self.y - 1),
        ]

    def __hash__(self) -> int:
        return hash(f"{self.x},{self.y}")


@dataclass
class Map:

    fields: List[List[int]]

    def get(self, point: Point) -> int:
        if point.x < 0 or len(self.fields) <= point.x:
            return -10
        if point.y < 0 or len(self.fields[0]) <= point.y:
            return -10

        return self.fields[point.x][point.y]

    def filter(self, v: int) -> List[Point]:
        """Find all highest points in the map."""
        heights = []
        for i, row in enumerate(self.fields):
            for j, point_value in enumerate(row):
                if point_value == v:
                    heights.append(Point(i, j))
        return heights


def parse(file_name: str) -> Map:
    with open(file_name) as f:
        return Map([list(map(int, list(row))) for row in f.read().strip().split("\n")])


def map_find_trailheads_ratings(map: Map) -> Dict[Point, int]:
    trail_ratings: Dict[Point, int] = {}

    points_visited: Set[Point] = set()

    points_queue: queue.Queue = queue.Queue()
    for point_height in map.filter(9):
        points_queue.put((9, point_height))

    while not points_queue.empty():
        _, point = points_queue.get()

        if point in points_visited:
            continue
        points_visited.add(point)

        point_rating = trail_ratings.get(point, 1)
        point_value = map.get(point)

        if point_value == 0:
            continue

        for point_next in point.nexts():
            point_next_value = map.get(point_next)
            if point_value - point_next_value == 1:
                points_queue.put((point_next_value, point_next))
                point_next_rating = trail_ratings.get(point_next, 0)
                trail_ratings[point_next] = point_rating + point_next_rating

    trailheads_ratings: Dict[Point, int] = {}
    for point_trailhead in map.filter(0):
        rating = trail_ratings.get(point_trailhead, 0)
        trailheads_ratings[point_trailhead] = rating

    return trailheads_ratings


def main():
    map = parse("./input.prod")
    trailheads_ratings = map_find_trailheads_ratings(map)
    # for trailhead, rating in trailheads_ratings.items():
    #     print(trailhead, rating)
    print(sum([score for score in trailheads_ratings.values()]))


if __name__ == "__main__":
    main()
