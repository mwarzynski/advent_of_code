from typing import Dict, Optional, List


class Node:
    name: str
    parent: Optional[object]
    children: Dict[str, object]
    size: int
    is_dir: bool

    def __init__(
        self, name: str, size: int, is_dir: bool, parent: object = None
    ) -> None:
        self.name = name
        self.children = {}
        self.parent = parent
        if is_dir:
            size = 0
        self.size = size
        self.is_dir = is_dir

    def cd(self, name: str):
        if name == "..":
            return self.parent
        if name not in self.children:
            raise Exception(f"dir {name} not exists")
        return self.children[name]

    def add_node(self, node):
        node.parent = self
        self.children[node.name] = node

    def get_size(self) -> int:
        if self.is_dir:
            return sum([n.get_size() for n in self.children.values()])
        return self.size

    def eval_size(self):
        if not self.is_dir:
            return
        for node in self.children.values():
            node.eval_size()
        self.size = sum([n.size for n in self.children.values()])


class FileTree:

    nodes: Dict[str, Node]

    def __init__(self) -> None:
        self.nodes = {"/": Node("/", parent=self, size=0, is_dir=True)}

    def cd(self, name: str) -> Node:
        if name == "/":
            return self.nodes["/"]
        raise Exception("root has no other folder than '/'")

    def eval_size(self):
        self.nodes["/"].eval_size()


def parse_file_tree(filename: str):
    root = FileTree()
    node = root

    with open(filename, "r") as f:
        for line in f.readlines():
            line = line.strip("\n")

            if line.startswith("$ "):
                # Command
                command = line.split(" ")
                if command[1] == "cd":
                    node = node.cd(command[2])
                elif command[1] == "ls":
                    pass
            else:
                # Command Output, assumes `ls` command output
                output = line.split(" ")
                is_dir = False
                size = 0
                if output[0] == "dir":
                    is_dir = True
                else:
                    size = int(output[0])
                name = output[1]
                new_node = Node(name, size, is_dir)
                node.add_node(new_node)

    return root


def get_dirs_of_size(parent: Node, less_than: int) -> List:
    size = []
    for _, node in parent.children.items():
        size += get_dirs_of_size(node, less_than)
    if parent.is_dir and parent.size <= less_than:
        size += [parent]
    return size


def get_min_folder_to_delete(
    size_max: int, parent: Node, needed_space: int = 4795677
) -> int:
    size = size_max
    for _, node in parent.children.items():
        ns = get_min_folder_to_delete(size_max, node, needed_space)
        if ns < size:
            size = ns
    if size != size_max:
        return size

    if parent.size >= needed_space:
        return parent.size

    return size_max


def main():
    ft = parse_file_tree("./inputs/day_7.txt")
    ft.eval_size()

    dirs = get_dirs_of_size(ft.nodes["/"], 100000)
    s = 0
    for dir in dirs:
        s += dir.size
    print(f"dirs_size: {s}")

    x = get_min_folder_to_delete(ft.nodes["/"].size, ft.nodes["/"])
    print(f"delete_size: {x}")

    # dirs_size: 1391690
    # delete_size: 5469168


if __name__ == "__main__":
    main()
