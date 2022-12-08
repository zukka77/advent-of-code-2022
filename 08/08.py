from pathlib import Path
from typing import List
from functools import reduce
import logging
import re

logging.basicConfig()
log = logging.getLogger(__name__)
log.setLevel(level=logging.DEBUG)


def first_question(input_data: str) -> int:
    field = []
    for line in input_data.splitlines():
        row = []
        for t in line:
            row.append(int(t))
        field.append(row)

    total_visible = 0
    for r in range(1, len(field) - 1):
        for c in range(1, len(field) - 1):
            visible = 4
            tree = field[r][c]
            for y in range(0, r):
                if field[y][c] >= tree:
                    visible -= 1
                    break
            for y in range(r + 1, len(field)):
                if field[y][c] >= tree:
                    visible -= 1
                    break
            for x in range(0, c):
                if field[r][x] >= tree:
                    visible -= 1
                    break
            for x in range(c + 1, len(field)):
                if field[r][x] >= tree:
                    visible -= 1
                    break
            if visible:
                total_visible += 1

    return total_visible + 4 * len(field) - 4


def second_question(input_data: str) -> int:
    field = []
    for line in input_data.splitlines():
        row = []
        for t in line:
            row.append(int(t))
        field.append(row)

    tw_max = 0
    for r in range(1, len(field) - 1):
        for c in range(1, len(field) - 1):
            tree = field[r][c]
            tw_l = tw_r = tw_u = tw_d = 0
            for y in range(r - 1, -1, -1):
                tw_u += 1
                if field[y][c] >= tree:
                    break
            for y in range(r + 1, len(field)):
                tw_d += 1
                if field[y][c] >= tree:
                    break
            for x in range(c - 1, -1, -1):
                tw_l += 1
                if field[r][x] >= tree:
                    break
            for x in range(c + 1, len(field)):
                tw_r += 1
                if field[r][x] >= tree:
                    break
            if tw_max < tw_l * tw_r * tw_u * tw_d:
                tw_max = tw_l * tw_r * tw_u * tw_d

    return tw_max


if __name__ == "__main__":

    input_data = (Path(__file__).parent / "INPUT").read_text("UTF8")
    print(first_question(input_data))
    print(second_question(input_data))
