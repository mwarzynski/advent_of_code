from dataclasses import dataclass
from typing import Set, List, Dict, Self, Optional, Tuple
from enum import Enum


class Direction(Enum):
    TOP = 1
    LEFT = 2
    RIGHT = 3
    BOTTOM = 4


@dataclass
class Point:
    x: int
    y: int

    def neighbour(self, direction: Direction) -> Self:
        if direction == Direction.TOP:
            return self.__class__(self.x - 1, self.y)
        elif direction == Direction.LEFT:
            return self.__class__(self.x, self.y - 1)
        elif direction == Direction.BOTTOM:
            return self.__class__(self.x + 1, self.y)
        elif direction == Direction.RIGHT:
            return self.__class__(self.x, self.y + 1)

    def __hash__(self) -> int:
        return hash(f"{self.x},{self.y}")


@dataclass
class PlantRegion:
    points: Set[Point]

    def area(self) -> int:
        return len(self.points)

    def perimeter(self) -> int:
        fences: Set[Tuple[Point, Point]] = set()

        for point in self.points:
            for _, direction in enumerate(Direction):
                point_next = point.neighbour(direction)
                if point_next not in self.points:
                    fences.add((point, point_next))

        return len(fences)


@dataclass
class Garden:
    fields: List[str]

    def plant_exists(self, point: Point) -> bool:
        return 0 <= point.x < len(self.fields) and 0 <= point.y < len(self.fields[0])

    def plant_get(self, point: Point) -> Optional[str]:
        if not self.plant_exists(point):
            return None
        return self.fields[point.x][point.y]

    def plant_identify_regions(self) -> List[PlantRegion]:
        new_plant_region_id: int = 0
        point_to_plant_region: Dict[Point, int] = {}
        plant_regions: Dict[int, Set[Point]] = {}

        def plant_dfs(plant: str, point: Point, plant_region_id: int, visited: Set[Point]):
            if not self.plant_exists(point):
                return

            visited.add(point)
            point_plant = self.plant_get(point)

            if point_plant and point_plant == plant:
                point_to_plant_region[point] = plant_region_id
                plant_regions[plant_region_id].add(point)
                for _, direction in enumerate(Direction):
                    point_neighbour = point.neighbour(direction)
                    if point_neighbour in visited:
                        continue
                    plant_dfs(plant, point.neighbour(direction), plant_region_id, visited)

        for i, row in enumerate(self.fields):
            for j, plant in enumerate(row):
                point = Point(i, j)

                if point in point_to_plant_region:
                    continue

                # Launch DFS to find out the whole Plant Region.
                plant_regions[new_plant_region_id] = {point}
                point_to_plant_region[point] = new_plant_region_id
                plant_dfs(plant, point, new_plant_region_id, {point})

                new_plant_region_id += 1

        return [PlantRegion(region) for region in plant_regions.values()]


def main():
    with open("./input.prod") as f:
        garden = Garden(f.read().strip().split("\n"))

    plant_regions = garden.plant_identify_regions()

    result = 0
    for region in plant_regions:
        result += region.area() * region.perimeter()

    print(result)


if __name__ == "__main__":
    main()
