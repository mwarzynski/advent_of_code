from typing import Tuple, List, Dict


def parse(file_name: str) -> Tuple[List[int], List[int]]:
    with open(file_name) as f:
        lines = [line.strip() for line in f if line.strip()]
    list1, list2 = zip(*(map(int, line.split()) for line in lines))
    return list(list1), list(list2)


def part1(file_name: str):
    list1, list2 = parse(file_name)
    result = sum(abs(a - b) for a, b in zip(sorted(list1), sorted(list2)))
    print(f"Part1: {result}")


def part2(file_name: str):
    list1, list2 = parse(file_name)
    occurrences: Dict[int, int] = {}
    for v in list2:
        occurrences[v] = occurrences.get(v, 0) + 1
    result = sum(v * occurrences.get(v, 0) for v in list1)
    print(f"Part2: {result}")

def main():
    file_name = "./input.prod"
    part1(file_name)
    part2(file_name)

if __name__ == "__main__":
    main()
