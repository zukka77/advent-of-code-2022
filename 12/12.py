from pathlib import Path
from typing import List, Callable
import logging
import math

logging.basicConfig()
log = logging.getLogger(__name__)
log.setLevel(level=logging.DEBUG)

"""
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"""

from collections import defaultdict
import heapq as heap

# https://levelup.gitconnected.com/dijkstra-algorithm-in-python-8f0e75e3f16e
def dijkstra(G, startingNode):
    visited = set()
    parentsMap = {}
    pq = []
    nodeCosts = defaultdict(lambda: float("inf"))
    nodeCosts[startingNode] = 0
    heap.heappush(pq, (0, startingNode))

    while pq:
        # go greedily by always extending the shorter cost nodes first
        _, node = heap.heappop(pq)
        visited.add(node)

        for adjNode in G[node]["neigh"]:
            weight = 1
            if adjNode in visited:
                continue

            newCost = nodeCosts[node] + weight
            if nodeCosts[adjNode] > newCost:
                parentsMap[adjNode] = node
                nodeCosts[adjNode] = newCost
                heap.heappush(pq, (newCost, adjNode))

    return parentsMap, nodeCosts


def first_question(input_data: str) -> int:
    field = []
    for y, line in enumerate(input_data.splitlines()):
        field.append([{"neigh": set(), "coord": (y, x), "val": c, "ordval": ord(c)} for x, c in enumerate(line)])
        if "S" in line:
            start = [y, line.index("S")]
            field[y][start[1]]["val"] = "a"
            field[y][start[1]]["ordval"] = ord("a")
        if "E" in line:
            end = [y, line.index("E")]
            field[y][end[1]]["val"] = "z"
            field[y][end[1]]["ordval"] = ord("z")
    ncols = len(line)
    nrows = len(field)
    # Build Graph
    G = {(y, x): field[y][x] for x in range(ncols) for y in range(nrows)}
    # Find neighbors
    for y in range(nrows):
        for x in range(ncols):
            for yy in range(max(0, y - 1), min(nrows, y + 2)):
                if field[yy][x]["ordval"] - field[y][x]["ordval"] <= 1 and y != yy:
                    field[y][x]["neigh"].add((yy, x))
            for xx in range(max(0, x - 1), min(ncols, x + 2)):
                if field[y][xx]["ordval"] - field[y][x]["ordval"] <= 1 and x != xx:
                    field[y][x]["neigh"].add((y, xx))
    from pprint import pprint

    _, nodeCosts = dijkstra(G, (start[0], start[1]))

    return nodeCosts[(end[0], end[1])]


def second_question(input_data: str) -> int:
    field = []
    for y, line in enumerate(input_data.splitlines()):
        field.append([{"neigh": set(), "coord": (y, x), "val": c, "ordval": ord(c)} for x, c in enumerate(line)])
        if "S" in line:
            start = [y, line.index("S")]
            field[y][start[1]]["val"] = "a"
            field[y][start[1]]["ordval"] = ord("a")
        if "E" in line:
            end = [y, line.index("E")]
            field[y][end[1]]["val"] = "z"
            field[y][end[1]]["ordval"] = ord("z")
    ncols = len(line)
    nrows = len(field)
    # Build Graph
    G = {(y, x): field[y][x] for x in range(ncols) for y in range(nrows)}
    # Find neighbors
    for y in range(nrows):
        for x in range(ncols):
            for yy in range(max(0, y - 1), min(nrows, y + 2)):
                if field[yy][x]["ordval"] - field[y][x]["ordval"] <= 1 and y != yy:
                    field[y][x]["neigh"].add((yy, x))
            for xx in range(max(0, x - 1), min(ncols, x + 2)):
                if field[y][xx]["ordval"] - field[y][x]["ordval"] <= 1 and x != xx:
                    field[y][x]["neigh"].add((y, xx))

    starting_points = [x["coord"] for x in G.values() if x["val"] == "a"]
    # get steps number for every starting point
    steps = [dijkstra(G, sp)[1][(end[0], end[1])] for sp in starting_points]
    return sorted(steps)[0]


if __name__ == "__main__":
    input_data = (Path(__file__).parent / "INPUT").read_text("UTF8")
    print(first_question(input_data))
    print(second_question(input_data))
