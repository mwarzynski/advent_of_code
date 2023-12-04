from dataclasses import dataclass


@dataclass
class G:
    pass

    def __init__(self) -> None:
        pass

    def run(self):
        pass


def parse_list(l):
    result = []
    d = 0
    nd = ""
    l = l[1:-1]

    for c in l:
        if c == "[":
            d += 1
        elif c == "]":
            d -= 1

        if not (c == "," and d == 0):
            nd += c

        if d == 0 and c != ",":
            try:
                v = int(nd)
                result.append(v)
            except ValueError:
                result.append(parse_list(nd))
                pass
            nd = ""

    return result


def lists_in_right_order(l1, l2) -> bool:
    for i in range(len(l1)):
        right_exists = False
        try:
            left = l1[i]
        except IndexError:
            pass
        try:
            right = l2[i]
            right_exists = True
        except IndexError:
            pass

        if not right_exists:
            return False

        if isinstance(left, int) and isinstance(right, int):
            # print(f"{left} <> {right} (int)")
            if left < right:
                return True
            if left > right:
                return False
            continue

        if isinstance(left, list) and isinstance(right, list):
            # print(f"{left} <> {right} (list - list)")
            if not lists_in_right_order(left, right):
                return False

        if isinstance(left, list) and isinstance(right, int):
            # print(f"{left} <> {right} (list - int)")
            if not lists_in_right_order(left, [right]):
                return False

        if isinstance(left, int) and isinstance(right, list):
            # print(f"{left} <> {right} (int - list)")
            if not lists_in_right_order([left], right):
                return False

    return True


def lists_sum_indices(l1):
    v = 0
    for i, e in enumerate(l1):
        if isinstance(e, int):
            v += e
        elif isinstance(e, list):
            v += lists_sum_indices(e)
    return v


def main():
    s = 0
    with open("input.dev") as f:
        data = f.read()
    list_pairs = data.split("\n\n")
    for i, lp in enumerate(list_pairs):
        l1, l2 = lp.split("\n")
        l1, l2 = parse_list(l1), parse_list(l2)
        v = lists_in_right_order(l1, l2)
        if v:
            s += i + 1
        print(l1)
        print(l2)
        print(v)
        print()
    print(s)


if __name__ == "__main__":
    main()
# 697
