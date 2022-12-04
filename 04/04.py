from pathlib import Path
from typing import List
from functools import reduce
import logging

logging.basicConfig()
log = logging.getLogger(__name__)
log.setLevel(level=logging.WARNING)


def first_question(input_data: str) -> int:
    lines = input_data.splitlines()
    contained_ranges = 0
    for l in lines:
        first_range, second_range = l.split(",")
        first_range = list(map(lambda x: int(x), first_range.split("-")))
        second_range = list(map(lambda x: int(x), second_range.split("-")))
        if (
            first_range[0] >= second_range[0]
            and first_range[1] <= second_range[1]
            or first_range[0] <= second_range[0]
            and first_range[1] >= second_range[1]
        ):
            contained_ranges += 1
        log.debug(f"first: {first_range} second:{second_range}")
    return contained_ranges


def second_question(input_data: str) -> int:
    lines = input_data.splitlines()
    overlaps = 0
    for l in lines:
        first_range, second_range = l.split(",")
        first_range = list(map(lambda x: int(x), first_range.split("-")))
        second_range = list(map(lambda x: int(x), second_range.split("-")))
        if (
            first_range[0] <= second_range[0]
            and first_range[1] <= second_range[1]
            and first_range[1] >= second_range[0]
            or first_range[0] >= second_range[0]
            and first_range[0] <= second_range[1]
            and first_range[1] >= second_range[1]
            or first_range[0] >= second_range[0]
            and first_range[1] <= second_range[1]
            or first_range[0] <= second_range[0]
            and first_range[1] >= second_range[1]
        ):
            overlaps += 1
            log.debug("OVERLAP")
        log.debug(f"first: {first_range} second:{second_range}")
    return overlaps


if __name__ == "__main__":

    input_data = (Path(__file__).parent / "INPUT").read_text("UTF8")
    print(first_question(input_data))
    print(second_question(input_data))
