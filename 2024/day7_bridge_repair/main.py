from typing import List

from dataclasses import dataclass


@dataclass
class Equation:
    test_value: int
    numbers: List[int]

    def can_be_true_part1(self) -> bool:
        def check_recursive(i: int, current_result: int) -> bool:
            if i == len(self.numbers):
                return current_result == self.test_value
            if i < len(self.numbers):
                a = check_recursive(i + 1, current_result * self.numbers[i])
                b = check_recursive(i + 1, current_result + self.numbers[i])
                return a or b
            return False

        return check_recursive(0, 0)

    def can_be_true_part2(self) -> bool:
        def check_recursive(i: int, current_result: int) -> bool:
            if i == len(self.numbers):
                return current_result == self.test_value
            if i < len(self.numbers):
                a = check_recursive(i + 1, current_result * self.numbers[i])
                b = check_recursive(i + 1, current_result + self.numbers[i])
                c = check_recursive(i + 1, int(str(current_result) + str(self.numbers[i])))
                return a or b or c
            return False

        return check_recursive(0, 0)


def parse(file_name: str) -> List[Equation]:
    with open(file_name) as f:
        items = [x.split(": ") for x in f.read().strip().split("\n")]
        return [Equation(test_value=int(y[0]), numbers=list(map(int, y[1].split(" ")))) for y in items]


def part1(file_name: str):
    equations = parse(file_name)
    print(sum([e.test_value for e in equations if e.can_be_true_part1()]))


def part2(file_name: str):
    equations = parse(file_name)
    print(sum([e.test_value for e in equations if e.can_be_true_part2()]))


def main():
    file_name = "./input.prod"
    part1(file_name)
    part2(file_name)
    pass


if __name__ == "__main__":
    main()
