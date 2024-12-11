from typing import List, Tuple


def stone_split(stone: int) -> Tuple[int, int]:
    stone_str = str(stone)
    i = int(len(stone_str) / 2)
    return int(stone_str[:i]), int(stone_str[i:])


def stones_iterate(stones: List[int]) -> List[int]:
    result: List[int] = []

    for stone in stones:
        if stone == 0:
            result.append(1)
        elif len(str(stone)) % 2 == 0:
            stone_l, stone_r = stone_split(stone)
            result.append(stone_l)
            result.append(stone_r)
        else:
            result.append(stone * 2024)

    return result


def main():
    with open("./input.prod") as f:
        stones = [int(stone) for stone in f.read().strip().split(" ")]

    for i in range(25):
        stones = stones_iterate(stones)
        print(i, len(stones))

    print(len(stones))


if __name__ == "__main__":
    main()
