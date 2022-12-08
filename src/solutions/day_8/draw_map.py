import numpy as np
import matplotlib.pyplot as plt


def do():
    fig, ax = plt.subplots()

    with open("./inputs/day_8.txt", "r") as f:
        data = f.read()

    numbers = [[int(v) for v in line] for line in data.split("\n")]
    matrix = np.matrix(numbers)

    ax.matshow(matrix, cmap=plt.cm.Greens)

    plt.savefig("forest.png")


if __name__ == "__main__":
    do()
