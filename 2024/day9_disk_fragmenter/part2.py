from typing import List, Tuple, Optional, Self
from copy import deepcopy


class DiskBlock:

    index: int
    length: int
    # file_id defines the File for which the Content is stored
    # if DiskBlock represents empty space, then this ID is None
    file_id: Optional[int]

    _children: List[Self]

    def __init__(self, index: int, file_id: Optional[int], length: int) -> None:
        self.index = index
        self.file_id = file_id
        self.length = length
        self._children = []

    def get(self) -> List[Self]:
        if not self._children:
            return [self]
        return self._children

    def index_range(self) -> Tuple[int, int]:
        """index_range returns [a,b) range for the block"""
        return (self.index, self.index + self.length)

    def reset(self):
        self.file_id = None

    def _leftfill(self, block: Self) -> bool:
        if self.file_id != None:
            return False

        if len(self._children) > 0:
            children_length = sum([b.length for b in self._children])
            length_free = self.length - children_length
            if length_free < block.length:
                return False

            block.index = self.index + children_length
            self._children.append(block)

            return True

        if block.length > self.length:
            return False

        if block.length == self.length:
            self.file_id = block.file_id
            return True

        block.index = self.index
        self._children.append(block)

        return True

    def leftfill(self, block: Self) -> bool:
        block_c = deepcopy(block)
        result = self._leftfill(block_c)
        return result

    def __str__(self) -> str:
        return f"DiskBlock(index={self.index},length={self.length},file_id={self.file_id})"


class Disk:

    blocks: List[DiskBlock]

    def __init__(self, memory_dense: str) -> None:
        self.blocks = self._parse_blocks_from_memory_dense(memory_dense)

    def _parse_blocks_from_memory_dense(self, memory_dense: str) -> List[DiskBlock]:
        blocks = []
        block_current_index = 0
        for i, block_length in enumerate(memory_dense):
            block_length = int(block_length)
            if block_length == 0:
                continue
            disk_block: DiskBlock
            if i % 2 == 0:
                file_id = int(i / 2)
                disk_block = DiskBlock(index=block_current_index, file_id=file_id, length=block_length)
            else:
                disk_block = DiskBlock(index=block_current_index, file_id=None, length=block_length)
            block_current_index += block_length
            blocks.append(disk_block)
        return blocks

    def fragment(self):
        blocks_free_indices = list([i for i, b in enumerate(self.blocks) if b.file_id == None])

        for i in range(len(self.blocks)):
            j = len(self.blocks) - i - 1
            block = self.blocks[j]

            if block.file_id == None:  # No need to leftfill the freespace block.
                continue

            for blocks_free_index in blocks_free_indices:
                if blocks_free_index >= j:
                    break

                block_free = self.blocks[blocks_free_index]
                if block_free.leftfill(block):
                    block.reset()
                    break

            # Speed up the leftfill checks by filtering out already used blocks.
            blocks_free_indices = [b for b in blocks_free_indices if self.blocks[b].file_id == None]

    def blocks_flat(self) -> List[DiskBlock]:
        blocks = []
        for block in self.blocks:
            if len(block._children) > 0:
                for child in block._children:
                    blocks.append(child)
            else:
                blocks.append(block)
        return blocks

    def print(self):
        i = 0
        for block in self.blocks_flat():
            index_range = block.index_range()
            while i < index_range[1]:
                if i >= index_range[0] and block.file_id != None:
                    print(block.file_id, end="")
                else:
                    print(".", end="")
                i += 1
        print("")

    def checksum2(self) -> int:
        checksum = 0
        i = 0

        for block in self.blocks_flat():
            index_range = block.index_range()
            while i < index_range[1]:
                if i >= index_range[0] and block.file_id != None:
                    checksum += i * block.file_id
                i += 1
        return checksum


def main():
    with open("./input.prod") as f:
        disk = Disk(memory_dense=f.read().strip())

    disk.fragment()
    # disk.print()

    print(disk.checksum2())


if __name__ == "__main__":
    main()
