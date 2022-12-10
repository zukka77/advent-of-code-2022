from pathlib import Path
from typing import List
from functools import reduce
import logging
from math import copysign
import sys

logging.basicConfig()
log = logging.getLogger(__name__)
log.setLevel(level=logging.DEBUG)


def first_question(input_data: str) -> int:
    X = 1
    strengths = []
    instruction_queue = []
    lines = input_data.splitlines()

    for (i, line) in enumerate(input_data.splitlines()):
        if len(instruction_queue):
            X += instruction_queue.pop(0)
        cycle = i + 1
        if cycle == 20 or (cycle - 20) % 40 == 0 and cycle <= 220:
            strengths.append(cycle * X)
        if line == "noop":
            instruction_queue.append(0)
        else:
            quantity = int(line.split()[1])
            instruction_queue = instruction_queue + [0, quantity]
    # DRAIN QUEUE
    while len(instruction_queue):
        X += instruction_queue.pop(0)
        cycle += 1
        if cycle == 20 or (cycle - 20) % 40 == 0 and cycle <= 220:
            strengths.append(cycle * X)
    return sum(strengths)


def second_question(input_data: str):
    X = 1
    instruction_queue = []
    lines = input_data.splitlines()
    cycle = 0
    for (i, line) in enumerate(input_data.splitlines()):
        if cycle % 40 == 0:
            sys.stdout.write("\n")
        if cycle // 40 == 6:
            break
        if len(instruction_queue):
            X += instruction_queue.pop(0)
        cycle = i + 1
        if abs(((cycle - 1) % 40) - X) <= 1:
            sys.stdout.write("#")
        else:
            sys.stdout.write(".")
        if line == "noop":
            instruction_queue.append(0)
        else:
            quantity = int(line.split()[1])
            instruction_queue = instruction_queue + [0, quantity]
    # DRAIN QUEUE
    while len(instruction_queue) and cycle // 40 < 6:
        if cycle % 40 == 0:
            sys.stdout.write("\n")
        X += instruction_queue.pop(0)
        cycle += 1
        if abs(((cycle - 1) % 40) - X) <= 1:
            sys.stdout.write("#")
        else:
            sys.stdout.write(".")


if __name__ == "__main__":

    input_data = (Path(__file__).parent / "INPUT").read_text("UTF8")
    print(first_question(input_data))
    second_question(input_data)
