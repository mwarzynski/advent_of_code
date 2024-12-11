from typing import List, Tuple, Dict


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


def stones_count_after_iters(stones: List[int], iterations: int) -> int:
    # Stone -> Iteration -> Count
    evaluations: Dict[int, Dict[int, int]] = {}

    def iter(stone: int, iteration: int) -> int:
        if iteration == 0:
            return 1
        stone_count = evaluations.get(stone, {}).get(iteration, None)
        if stone_count:
            return stone_count

        stones_next = stones_iterate([stone])
        result = 0
        for stone_next in stones_next:
            result += iter(stone_next, iteration - 1)

        stone_evals = evaluations.get(stone, {})
        stone_evals[iteration] = result
        evaluations[stone] = stone_evals
        return result

    result = 0
    for stone in stones:
        result += iter(stone, iterations)

    return result


def stones_count(stones: List[int]) -> Dict[int, int]:
    result = {}
    for stone in stones:
        c = result.get(stone, 0)
        result[stone] = c + 1
    return result


def main():
    with open("./input.prod") as f:
        stones = [int(stone) for stone in f.read().strip().split(" ")]
    result = stones_count_after_iters(stones, 75)
    print(result)


if __name__ == "__main__":
    main()
