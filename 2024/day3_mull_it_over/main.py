import re

EXPRESSION = "mul\\((\\d{1,3})\\,(\\d{1,3})\\)"


def part1(data: str) -> int:
    """Find all occurrences of mul(x,y) in the code, mulitply x*y and sum the results"""
    operations = [list(map(int, x)) for x in re.findall(EXPRESSION, data)]
    return sum([a * b for (a, b) in operations])


def part2(data: str) -> int:
    """Same as part1, but discard the disabled parts of the code. Block starts as do()."""
    donts = data.split("don't()")
    code = "".join([donts[0]] + [item.split("do()", 1)[-1] for item in donts[1:] if "do()" in item])
    return part1(code)


def main():
    with open("./input.prod") as f:
        data = f.read()
    print(f"{part1(data)}")
    print(f"{part2(data)}")


if __name__ == "__main__":
    main()
