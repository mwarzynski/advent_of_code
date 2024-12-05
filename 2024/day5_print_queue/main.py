from typing import List, Tuple, Dict, Set
from dataclasses import dataclass


@dataclass
class ManualUpdate:
    pages: List[int]

    def middle_page(self) -> int:
        return self.pages[int(len(self.pages) / 2)]


@dataclass
class PageOrder:
    rules: List[Tuple[int, int]]

    def _order_set(self) -> Dict[int, Set[int]]:
        order_set = {}

        for a, b in self.rules:
            if a in order_set:
                s = order_set[a]
                s.add(b)
                order_set[a] = s
            else:
                order_set[a] = set([b])

        return order_set

    def fix_update(self, update: ManualUpdate) -> ManualUpdate:
        """Method returns a fixed copy of provided ManualUpdate, which satisfies all Rules."""
        # Filter rules to only include pages in the update.
        relevant_rules = [(a, b) for a, b in self.rules if a in update.pages and b in update.pages]

        # Build a directed graph for the relevant rules.
        # Graph directs from B to A, where A must be before B in the fixed ManualUpdate.
        dependency_graph = {page: set() for page in update.pages}
        for a, b in relevant_rules:
            dependency_graph[b].add(a)

        # Perform DFS to find out the page numbers in the sorted order.
        # Items are added to sorted_pages in the order from 'last reached' to 'first reached'.
        visited = set()
        sorted_pages = []

        def visit(page):
            if page in visited:
                return
            visited.add(page)
            for dependency in dependency_graph.get(page, []):
                visit(dependency)
            sorted_pages.append(page)

        for page in update.pages:
            visit(page)

        return ManualUpdate(pages=sorted_pages)

    def is_update_correct(self, update: ManualUpdate) -> bool:
        order_set = self._order_set()

        previous_numbers = set()
        for page_number in update.pages:
            if page_number in order_set:
                for forbidden_page_number in order_set[page_number]:
                    if forbidden_page_number in previous_numbers:
                        return False
            previous_numbers.add(page_number)

        return True


@dataclass
class Queue:
    page_order: PageOrder
    updates: List[ManualUpdate]


def parse(file_name: str) -> Queue:
    with open(file_name) as f:
        data = f.read().strip()
        rules_raw, pages_raw = data.split("\n\n")

        rules: List[Tuple[int, int]] = [
            (int(a), int(b)) for line in rules_raw.split("\n") for a, b in [line.split("|")]
        ]
        updates_pages = [list(map(int, x.split(","))) for x in pages_raw.split("\n")]

        return Queue(PageOrder(rules), [ManualUpdate(pages) for pages in updates_pages])


def part1(file_name: str):
    queue = parse(file_name)

    result = 0
    for update in queue.updates:
        if queue.page_order.is_update_correct(update):
            result += update.middle_page()

    print(result)  # dev: 143, prod: 6498


def part2(file_name: str):
    queue = parse(file_name)

    result = 0
    for update in queue.updates:
        if not queue.page_order.is_update_correct(update):
            fixed_update = queue.page_order.fix_update(update)
            result += fixed_update.middle_page()

    print(result)  # dev: 123, prod: 5017


def main():
    file_name = "input.prod"
    part1(file_name)
    part2(file_name)


if __name__ == "__main__":
    main()
