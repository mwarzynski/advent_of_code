from typing import List, Tuple


def parse(file_name: str) -> List[List[int]]:
    with open(file_name) as f:
        return [list(map(int, line.split())) for line in f if line.strip()]


def divide_into_pairs(items: List[int]) -> List[Tuple[int, int]]:
    return [(items[i], items[i + 1]) for i in range(0, len(items) - 1)]


def is_report_safe(report: List[int]) -> bool:
    all_increasing, all_decreasing, level_difference_satisfied = True, True, True
    for a, b in divide_into_pairs(report):
        if a < b:
            all_increasing = False
        elif a > b:
            all_decreasing = False
        if abs(a - b) > 3 or abs(a - b) == 0:
            level_difference_satisfied = False
    return (all_increasing or all_decreasing) and level_difference_satisfied


def part1(file_name: str):
    reports = parse(file_name)
    safe_reports = sum(is_report_safe(report) for report in reports)
    print(safe_reports)


def part2(file_name: str):
    reports = parse(file_name)

    safe_reports = 0
    for report in reports:
        if is_report_safe(report):
            safe_reports += 1
            continue

        for i in range(len(report)):
            modified_report = report[:i] + report[i + 1 :]
            if is_report_safe(modified_report):
                safe_reports += 1
                break

    print(safe_reports)


def main():
    file_name = "./input.prod"
    part1(file_name)
    part2(file_name)


if __name__ == "__main__":
    main()
