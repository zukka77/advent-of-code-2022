from pathlib import Path
from typing import List
from functools import reduce
import logging

logging.basicConfig()
log = logging.getLogger(__name__)
log.setLevel(level=logging.WARNING)


def get_priority(elem: str) -> int:
    val = ord(elem) - ord("a") + 1
    return val if val > 0 else ord(elem) - ord("A") + 27


def first_question(input_data: str) -> int:
    lines = input_data.splitlines()
    prio_sum = 0
    for l in lines:
        first_half_set = set(l[: len(l) // 2])
        second_half_set = set(l[len(l) // 2 :])
        common_element = (first_half_set & second_half_set).pop()
        prio_sum += get_priority(common_element)
        log.debug(f"elem: {common_element} prio:{get_priority(common_element)}")
    return prio_sum


def second_question(input_data: str) -> int:
    lines = input_data.splitlines()
    prio_sum = 0
    for n in range(0, len(lines), 3):
        sets = [set(l) for l in lines[n : n + 3]]
        log.debug(sets)
        common_element = reduce(lambda v, a: v & a, sets).pop()
        prio_sum += get_priority(common_element)
        log.debug(f"elem: {common_element} prio:{get_priority(common_element)}")
    return prio_sum


if __name__ == "__main__":

    input_data = (Path(__file__).parent / "INPUT").read_text("UTF8")
    print(first_question(input_data))
    print(second_question(input_data))
