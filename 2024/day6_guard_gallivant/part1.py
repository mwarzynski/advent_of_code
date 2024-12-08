from typing import List, Set
from dataclasses import dataclass
from enum import Enum


class Direction(Enum):
    UP = 1
    RIGHT = 2
    DOWN = 3
    LEFT = 4


def turn(d: Direction) -> Direction:
    if d == Direction.UP:
        return Direction.RIGHT
    elif d == Direction.RIGHT:
        return Direction.DOWN
    elif d == Direction.DOWN:
        return Direction.LEFT
    elif d == Direction.LEFT:
        return Direction.UP


@dataclass
class Point:
    x: int  # Row
    y: int  # Column

    def __hash__(self):
        return hash(f"{self.x}{self.y}")

    def go(self, direction: Direction):
        if direction == Direction.UP:
            return Point(x=self.x - 1, y=self.y)
        elif direction == Direction.RIGHT:
            return Point(x=self.x, y=self.y + 1)
        elif direction == Direction.DOWN:
            return Point(x=self.x + 1, y=self.y)
        elif direction == Direction.LEFT:
            return Point(x=self.x, y=self.y - 1)


@dataclass
class Position:
    point: Point
    direction: Direction


class Map:
    fields: List[str]
    _init_pos: Position

    def __init__(self, fields: List[str]):
        self.fields = fields
        for i, row in enumerate(fields):
            for j, v in enumerate(row):
                if v == "^":
                    self._pos = Position(Point(x=i, y=j), direction=Direction.UP)
                elif v == ">":
                    self._pos = Position(Point(x=i, y=j), direction=Direction.RIGHT)
                elif v == "v":
                    self._pos = Position(Point(x=i, y=j), direction=Direction.DOWN)
                elif v == "<":
                    self._pos = Position(Point(x=i, y=j), direction=Direction.LEFT)

        # Replace position char with '.', such that we have only '.' and '#' in the map.
        self.fields[self._pos.point.x] = (
            self.fields[self._pos.point.x][: self._pos.point.y]
            + "."
            + self.fields[self._pos.point.x][self._pos.point.y + 1 :]
        )

    def get(self, point: Point) -> str:
        return self.fields[point.x][point.y]

    def is_outside(self, point: Point) -> bool:
        if point.x < 0 or len(self.fields) <= point.x:
            return True
        if point.y < 0 or len(self.fields[0]) <= point.y:
            return True
        return False


def parse(file_name: str) -> Map:
    with open(file_name) as f:
        return Map(f.read().strip().split("\n"))


def part1(file_name: str):
    map = parse(file_name)

    visited_fields: Set[Point] = set()
    current_point = map._pos.point
    visited_fields.add(current_point)
    direction = map._pos.direction

    while True:
        next_point = current_point.go(direction)
        if map.is_outside(next_point):
            break

        next_field = map.get(next_point)

        if next_field == "#":
            direction = turn(direction)

        elif next_field == ".":
            current_point = next_point
            visited_fields.add(next_point)

    print(len(visited_fields))


def main():
    file_name = "./input.prod"
    part1(file_name)


if __name__ == "__main__":
    main()
