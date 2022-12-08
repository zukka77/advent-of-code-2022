from pathlib import Path
from typing import List
from functools import reduce
import logging
import re

logging.basicConfig()
log = logging.getLogger(__name__)
log.setLevel(level=logging.DEBUG)


def first_question(input_data: str) -> int:
    dir_sizes = {}
    cwd = "/"
    for line in input_data.splitlines():
        if not line:
            continue
        line = line.strip()
        if line.startswith("$ cd"):
            d = line.split()[2]
            if d == "/":
                cwd = "/"
            elif d == "..":
                cwd = "/".join(cwd.split("/")[:-1])
            else:
                if cwd == "/":
                    cwd = f"/{d}"
                else:
                    cwd += f"/{d}"
            dir_sizes[cwd] = dir_sizes.get(cwd, 0)  # dir with no files
        elif line.startswith("$ ls") or line.startswith("dir"):
            continue
        else:
            size = int(line.split()[0])
            dir_sizes[cwd] = dir_sizes.get(cwd, 0) + size

    total_sizes = {}
    for d in dir_sizes:
        total_sizes[d] = sum(map(lambda x: x[1], filter(lambda x: x[0].startswith(d), dir_sizes.items())))

    return sum(map(lambda x: x[1], filter(lambda x: x[1] <= 100_000, total_sizes.items())))


def first_question2(input_data: str) -> int:
    files = {}
    dirs = set()
    cwd = "/"
    for line in input_data.splitlines():
        if not line:
            continue
        line = line.strip()
        if line.startswith("$ cd"):
            d = line.split()[2]
            if d == "/":
                cwd = "/"
            elif d == "..":
                cwd = "/".join(cwd.split("/")[:-1])
            else:
                if cwd == "/":
                    cwd = f"/{d}"
                else:
                    cwd += f"/{d}"
            dirs.add(cwd)
        elif line.startswith("$ ls") or line.startswith("dir"):
            continue
        else:
            (size, filename) = line.split()
            size = int(size)
            files[cwd + "/" + filename] = size
            dirs.add(cwd)

    total_sizes = {}
    for d in dirs:
        total_sizes[d] = sum(map(lambda x: x[1] if "/".join(x[0].split("/")[:-1]).startswith(d) else 0, files.items()))

    return sum(map(lambda x: x[1], filter(lambda x: x[1] <= 100000, total_sizes.items())))


def second_question(input_data: str) -> int:
    dir_sizes = {}
    cwd = "/"
    for line in input_data.splitlines():
        if not line:
            continue
        line = line.strip()
        if line.startswith("$ cd"):
            d = line.split()[2]
            if d == "/":
                cwd = "/"
            elif d == "..":
                cwd = "/".join(cwd.split("/")[:-1])
            else:
                if cwd == "/":
                    cwd = f"/{d}"
                else:
                    cwd += f"/{d}"
            dir_sizes[cwd] = dir_sizes.get(cwd, 0)
        elif line.startswith("$ ls") or line.startswith("dir"):
            continue
        else:
            size = int(line.split()[0])
            dir_sizes[cwd] = dir_sizes.get(cwd, 0) + size

    total_sizes = {}
    for d in dir_sizes:
        total_sizes[d] = sum(map(lambda x: x[1], filter(lambda x: x[0].startswith(d), dir_sizes.items())))
    needed_space = 30_000_000 - (70_000_000 - total_sizes["/"])
    return list(filter(lambda x: x[1] >= needed_space, sorted(total_sizes.items(), key=lambda x: x[1])))[0][1]


if __name__ == "__main__":

    input_data = (Path(__file__).parent / "INPUT").read_text("UTF8")
    print(first_question(input_data))
    print(first_question2(input_data))
    print(second_question(input_data))
