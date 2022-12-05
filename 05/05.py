from pathlib import Path
from typing import List
from functools import reduce
import logging
import re

logging.basicConfig()
log = logging.getLogger(__name__)
log.setLevel(level=logging.WARNING)


def move_crates(instruction: str, stacks: dict):
    log.debug(instruction)
    moves_pattern = re.compile(r"move (?P<N>\d+) from (?P<FROM>\d+) to (?P<TO>\d+)")
    m = moves_pattern.search(instruction)
    n = int(m.group("N"))
    from_stack = int(m.group("FROM"))
    to_stack = int(m.group("TO"))
    log.debug(f"moving {n} crates from stack {from_stack} to stack {to_stack}")
    log.debug("FROM")
    log.debug(stacks)
    for _ in range(n):
        stacks[to_stack] += stacks[from_stack][-1:]
        stacks[from_stack] = stacks[from_stack][:-1]
    log.debug("TO")
    log.debug(stacks)


def move_crates2(instruction: str, stacks: dict):
    log.debug(instruction)
    moves_pattern = re.compile(r"move (?P<N>\d+) from (?P<FROM>\d+) to (?P<TO>\d+)")
    m = moves_pattern.search(instruction)
    n = int(m.group("N"))
    from_stack = int(m.group("FROM"))
    to_stack = int(m.group("TO"))
    log.debug(f"moving {n} crates from stack {from_stack} to stack {to_stack}")
    log.debug("FROM")
    log.debug(stacks)
    stacks[to_stack] += stacks[from_stack][-n:]
    stacks[from_stack] = stacks[from_stack][:-n]
    log.debug("TO")
    log.debug(stacks)


def first_question(input_data: str) -> str:
    lines = input_data.splitlines()
    stacks = {}
    # GET STACKS
    index = 0
    for l in lines:
        index += 1
        if not l:
            break
        for i in range(len(l)):
            if l[i] in (" ", "[", "]"):
                continue
            # current stack index is i//4+1
            stacks[i // 4 + 1] = stacks.get(i // 4 + 1, [])
            stacks[i // 4 + 1].insert(0, l[i])
    log.debug(stacks)
    # MOVE CRATES
    for l in lines[index:]:
        move_crates(l, stacks)
    log.debug(stacks)
    # return stacks top
    return "".join([stacks[k][-1] for k in sorted(stacks.keys())])


def second_question(input_data: str) -> str:
    lines = input_data.splitlines()
    stacks = {}
    # GET STACKS
    index = 0
    for l in lines:
        index += 1
        if not l:
            break
        for i in range(len(l)):
            if l[i] in (" ", "[", "]"):
                continue
            # current stack index is i//4+1
            stacks[i // 4 + 1] = stacks.get(i // 4 + 1, [])
            stacks[i // 4 + 1].insert(0, l[i])
    log.debug(stacks)
    # MOVE CRATES
    for l in lines[index:]:
        move_crates2(l, stacks)
    log.debug(stacks)
    # return stacks top
    return "".join([stacks[k][-1] for k in sorted(stacks.keys())])


if __name__ == "__main__":

    input_data = (Path(__file__).parent / "INPUT").read_text("UTF8")
    print(first_question(input_data))
    print(second_question(input_data))
