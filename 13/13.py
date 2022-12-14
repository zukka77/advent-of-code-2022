from pathlib import Path
from typing import List, Callable
import logging
import math

logging.basicConfig()
log = logging.getLogger(__name__)
log.setLevel(level=logging.DEBUG)
from itertools import chain


def unroll_empty(x, steps=0):
    if hasattr(x, "__iter__"):
        if not x:
            yield steps + 1
        for e in x:
            if hasattr(e, "__iter__"):
                yield from unroll_empty(e, steps + 2)
            else:
                yield steps + 1
    else:
        yield steps


def unroll(x):
    if hasattr(x, "__iter__"):
        if not x:
            yield 0
        for e in x:
            if hasattr(e, "__iter__"):
                yield from unroll(e)
            else:
                yield e
    else:
        yield x


def get_left(x):
    if hasattr(x, "__iter__"):
        if x:
            for e in x:
                if hasattr(e, "__iter__"):
                    yield from get_left(e)
                else:
                    yield e


def first_question(input_data: str) -> int:
    i = 1
    ok_indexes = 0
    for pair in input_data.split("\n\n"):
        left, right = [eval(p) for p in pair.split("\n")]
        print(f"Pair: {i}\n{left} # {right}")
        uleft = list(get_left(left))
        uright = list(get_left(right))
        print(f"{uleft} # {uright}")
        if uleft < uright:
            ok_indexes += 1
            print("\t<")
        else:
            print("\t>")

        i += 1
    return ok_indexes


def second_question(input_data: str) -> int:
    pass


if __name__ == "__main__":
    input_data = (Path(__file__).parent / "INPUT_EXAMPLE").read_text("UTF8")
    print(first_question(input_data))
    print(second_question(input_data))
