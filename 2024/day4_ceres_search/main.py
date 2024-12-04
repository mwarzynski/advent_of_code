from typing import List, Iterator


def parse(file_name: str) -> List[str]:
    with open(file_name) as f:
        return [line.strip() for line in f if line.strip()]


def iter_cols(rows: List[str]) -> Iterator[str]:
    for column in zip(*rows):
        yield "".join(column)


def iter_diags(rows: List[str], reverse: bool = False) -> Iterator[str]:
    n = len(rows)
    if n == 0:
        return

    m = len(rows[0])

    # n - length of a column
    # m - length of a row

    min = -n + 1
    max = m
    if reverse:
        min = 0
        max = n + m - 1

    for index_sum in range(min, max):
        diagonal = []
        for ni in range(0, n):
            for mi in range(0, m):
                current_index_sum = ni - mi
                if reverse:
                    current_index_sum = ni + mi
                if index_sum == current_index_sum:
                    diagonal.append(rows[ni][mi])

        yield "".join(diagonal)


def count_xmas(v: str) -> int:
    return v.count("XMAS") + v.count("SAMX")


def part1(file_name: str):
    """Solve part 1 by counting 'XMAS' and 'SAMX' in rows, columns, and diagonals."""
    rows = parse(file_name)
    result = sum(
        count_xmas(sequence)
        for sequence in rows + list(iter_cols(rows)) + list(iter_diags(rows)) + list(iter_diags(rows, reverse=True))
    )
    print(result)


def check_xmas_diagonal_pattern(rows: List[str], i: int, j: int, pattern: List[tuple]) -> bool:
    for di, dj, expected in pattern:
        ni, nj = i + di, j + dj
        if not (0 <= ni < len(rows) and 0 <= nj < len(rows[0]) and rows[ni][nj] == expected):
            return False
    return True


def part2(file_name: str):
    rows = parse(file_name)
    patterns = [
        [(-1, -1, "S"), (1, -1, "S"), (-1, 1, "M"), (1, 1, "M")],  # right
        [(-1, -1, "M"), (1, -1, "M"), (-1, 1, "S"), (1, 1, "S")],  # left
        [(-1, -1, "S"), (1, -1, "M"), (-1, 1, "S"), (1, 1, "M")],  # bottom
        [(-1, -1, "M"), (1, -1, "S"), (-1, 1, "M"), (1, 1, "S")],  # top
    ]

    result = 0
    for i in range(1, len(rows) - 1):
        for j in range(1, len(rows[i]) - 1):
            if rows[i][j] == "A" and any(check_xmas_diagonal_pattern(rows, i, j, pattern) for pattern in patterns):
                result += 1
    print(result)


def main():
    file_name = "./input.prod"
    part1(file_name)
    part2(file_name)


if __name__ == "__main__":
    main()
