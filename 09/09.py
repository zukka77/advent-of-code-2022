from pathlib import Path
from typing import List
from functools import reduce
import logging
from math import copysign

logging.basicConfig()
log = logging.getLogger(__name__)
log.setLevel(level=logging.DEBUG)


def move_up(head_position: List[int], tail_position: List[int], steps: int, tail_positions: set):
    for _ in range(steps):
        head_position[1] += 1
        move_tail(head_position, tail_position, tail_positions)


def move_down(head_position: List[int], tail_position: List[int], steps: int, tail_positions: set):
    for _ in range(steps):
        head_position[1] -= 1
        move_tail(head_position, tail_position, tail_positions)


def move_right(head_position: List[int], tail_position: List[int], steps: int, tail_positions: set):
    for _ in range(steps):
        head_position[0] += 1
        move_tail(head_position, tail_position, tail_positions)


def move_left(head_position: List[int], tail_position: List[int], steps: int, tail_positions: set):
    for _ in range(steps):
        head_position[0] -= 1
        move_tail(head_position, tail_position, tail_positions)


def move_tail_old(head_position: List[int], tail_position: List[int], tail_positions: set):
    if head_position[1] - tail_position[1] > 1:  # UP
        if head_position[0] == tail_position[0]:
            tail_position[1] += 1
        elif head_position[0] > tail_position[0]:
            tail_position[1] += 1
            tail_position[0] += 1
        else:
            tail_position[1] += 1
            tail_position[0] -= 1
    if head_position[1] - tail_position[1] < -1:  # DOWN
        if head_position[0] == tail_position[0]:
            tail_position[1] -= 1
        elif head_position[0] > tail_position[0]:
            tail_position[1] -= 1
            tail_position[0] += 1
        else:
            tail_position[1] -= 1
            tail_position[0] -= 1
    if head_position[0] - tail_position[0] > 1:  # RIGHT
        if head_position[1] == tail_position[1]:
            tail_position[0] += 1
        elif head_position[1] > tail_position[1]:
            tail_position[0] += 1
            tail_position[1] += 1
        else:
            tail_position[0] += 1
            tail_position[1] -= 1
    if head_position[0] - tail_position[0] < -1:  # LEFT
        if head_position[1] == tail_position[1]:
            tail_position[0] -= 1
        elif head_position[1] > tail_position[1]:
            tail_position[0] -= 1
            tail_position[1] += 1
        else:
            tail_position[0] -= 1
            tail_position[1] -= 1
    tail_positions.add((tail_position[0], tail_position[1]))


def move_tail(head_position: List[int], tail_position: List[int], tail_positions: set):
    if abs(head_position[0] - tail_position[0]) > 1 or abs(head_position[1] - tail_position[1]) > 1:
        if head_position[0] != tail_position[0]:
            tail_position[0] += copysign(1, head_position[0] - tail_position[0])
        if head_position[1] != tail_position[1]:
            tail_position[1] += copysign(1, head_position[1] - tail_position[1])
        tail_positions.add((tail_position[0], tail_position[1]))


def first_question(input_data: str) -> int:
    head_position = [0, 0]
    tail_position = [0, 0]
    tail_positions = {(0, 0)}
    for line in input_data.splitlines():
        (direction, steps) = line.split()
        match direction:
            case "U":
                move_up(head_position, tail_position, int(steps), tail_positions)
            case "D":
                move_down(head_position, tail_position, int(steps), tail_positions)
            case "R":
                move_right(head_position, tail_position, int(steps), tail_positions)
            case "L":
                move_left(head_position, tail_position, int(steps), tail_positions)
    return len(tail_positions)


def second_question(input_data: str) -> int:
    rope_positions = [[0, 0] for x in range(10)]
    tail_positions = [{(0, 0)} for x in range(10)]
    for line in input_data.splitlines():
        (direction, steps) = line.split()
        for _ in range(int(steps)):
            head_position = rope_positions[0]
            tail_position = rope_positions[1]
            match direction:
                case "U":
                    move_up(head_position, tail_position, 1, tail_positions[1])
                case "D":
                    move_down(head_position, tail_position, 1, tail_positions[1])
                case "R":
                    move_right(head_position, tail_position, 1, tail_positions[1])
                case "L":
                    move_left(head_position, tail_position, 1, tail_positions[1])
            for k in range(1, len(rope_positions) - 1):
                move_tail(rope_positions[k], rope_positions[k + 1], tail_positions[k + 1])
    return len(tail_positions[-1])


if __name__ == "__main__":

    input_data = (Path(__file__).parent / "INPUT").read_text("UTF8")
    print(first_question(input_data))
    print(second_question(input_data))
