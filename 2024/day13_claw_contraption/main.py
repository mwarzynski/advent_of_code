from typing import List, Optional, Self, Set
from dataclasses import dataclass

from functools import total_ordering
from heapq import heappop, heappush
from math import sqrt


@total_ordering
@dataclass
class Point:
    x: int
    y: int

    def distance(self, target: Self) -> int:
        a = abs(self.x - target.x)
        b = abs(self.y - target.y)
        return int(sqrt(a**2 + b**2))

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Point):
            return False
        return self.x == other.x and self.y == other.y

    def __lt__(self, other: "Point") -> bool:
        # Define a consistent ordering: prioritize x, then y
        if self.x != other.x:
            return self.x < other.x
        return self.y < other.y

    def __hash__(self) -> int:
        return hash(f"{self.x},{self.y}")


@dataclass
class Game:
    button_a_change: Point
    button_b_change: Point
    BUTTON_A_COST = 3
    BUTTON_B_COST = 1

    prize: Point

    def evaluate(self) -> Optional[int]:
        pq = []
        start = Point(0, 0)
        heappush(pq, (0, start))

        visited: Set[Point] = set()

        while pq:
            cost, current = heappop(pq)
            if current == self.prize:
                return cost
            if current.x > self.prize.x and current.y > self.prize.y:
                continue

            if current in visited:
                continue
            visited.add(Point(current.x, current.y))

            next_a = Point(current.x + self.button_a_change.x, current.y + self.button_a_change.y)
            next_b = Point(current.x + self.button_b_change.x, current.y + self.button_b_change.y)

            # case 1: button a press
            if Point(next_a.x, next_a.y) not in visited:
                heappush(pq, (cost + self.BUTTON_A_COST, next_a))
            # case 2: button a press
            if Point(next_b.x, next_b.y) not in visited:
                heappush(pq, (cost + self.BUTTON_B_COST, next_b))

        return None


def parse(file_name: str) -> List[Game]:
    with open(file_name) as f:
        games_raw = f.read().strip().split("\n\n")
        games = []
        for g in games_raw:
            parts = g.split("\n")
            button_a_parts = parts[0].removeprefix("Button A: ").split(", ")
            button_b_parts = parts[1].removeprefix("Button B: ").split(", ")
            prize_parts = parts[2].removeprefix("Prize: ").split(", ")

            button_a = Point(x=int(button_a_parts[0].removeprefix("X+")), y=int(button_a_parts[1].removeprefix("Y+")))
            button_b = Point(x=int(button_b_parts[0].removeprefix("X+")), y=int(button_b_parts[1].removeprefix("Y+")))
            prize = Point(x=int(prize_parts[0].removeprefix("X=")), y=int(prize_parts[1].removeprefix("Y=")))

            games.append(Game(button_a, button_b, prize))

    return games


def main():
    file_name = "./input.dev"
    games = parse(file_name)
    result = 0
    for game in games:
        cost = game.evaluate()
        if cost:
            result += cost
    print(result)


if __name__ == "__main__":
    main()
