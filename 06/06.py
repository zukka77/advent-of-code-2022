from pathlib import Path
from typing import List
from functools import reduce
import logging
import re

logging.basicConfig()
log = logging.getLogger(__name__)
log.setLevel(level=logging.WARNING)


def first_question(input_data: str) -> int:
    data = input_data
    for i in range(0, len(data) - 4):
        char_set = set(data[i : i + 4])
        if len(char_set) == 4:
            return i + 4


def second_question(input_data: str) -> int:
    data = input_data
    for i in range(0, len(data) - 14):
        char_set = set(data[i : i + 14])
        if len(char_set) == 14:
            return i + 14


if __name__ == "__main__":

    input_data = (Path(__file__).parent / "INPUT").read_text("UTF8")
    print(first_question(input_data))
    print(second_question(input_data))
