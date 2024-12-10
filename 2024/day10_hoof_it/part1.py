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
            return -1
        if point.y < 0 or len(self.fields[0]) <= point.y:
            return -1

        return self.fields[point.x][point.y]

    def heights(self) -> List[Point]:
        """Find all highest points in the map."""
        heights = []
        for i, row in enumerate(self.fields):
            for j, v in enumerate(row):
                if v == 9:
                    heights.append(Point(i, j))
        return heights


def parse(file_name: str) -> Map:
    with open(file_name) as f:
        return Map([list(map(int, list(row))) for row in f.read().strip().split("\n")])


def map_find_trailheads_from_height(map: Map, point_height: Point) -> List[Point]:
    assert map.get(point_height) == 9

    points_heads: Set[Point] = set()

    points_queue: queue.Queue = queue.Queue()
    points_queue.put(point_height)

    while not points_queue.empty():
        point = points_queue.get()
        point_value = map.get(point)

        if point_value == 0:
            points_heads.add(point)
            continue

        for point_next in point.nexts():
            if point_value - map.get(point_next) == 1:
                points_queue.put(point_next)

    return list(points_heads)


def main():
    map = parse("./input.prod")

    trailhead_scores: Dict[Point, int] = {}

    for point_height in map.heights():
        trailheads = map_find_trailheads_from_height(map, point_height)
        for trailhead in trailheads:
            score = trailhead_scores.get(trailhead, 0)
            score += 1
            trailhead_scores[trailhead] = score

    # for trailhead, score in trailhead_scores.items():
    #     print(trailhead, score)

    print(sum([score for score in trailhead_scores.values()]))


if __name__ == "__main__":
    main()
