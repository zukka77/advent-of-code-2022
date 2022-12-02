from pathlib import Path
from typing import List

POINTS = {"ROCK": 1, "PAPER": 2, "SCISSORS": 3, "X": 1, "Y": 2, "Z": 3}


def play_round(opponent: str, your: str) -> int:
    if (opponent == "A" and your == "X") or (opponent == "B" and your == "Y") or (opponent == "C" and your == "Z"):
        return 3 + POINTS[your]
    if (opponent == "A" and your == "Z") or (opponent == "B" and your == "X") or (opponent == "C" and your == "Y"):
        return POINTS[your]
    if (opponent == "A" and your == "Y") or (opponent == "B" and your == "Z") or (opponent == "C" and your == "X"):
        return 6 + POINTS[your]
    raise ValueError(f"Combination not found {opponent} {your}")


def play_round2(opponent: str, outcome: str) -> int:
    if outcome == "X":  # DEFEAT
        if opponent == "A":
            return POINTS["SCISSORS"]
        if opponent == "B":  # PAPER
            return POINTS["ROCK"]
        return POINTS["PAPER"]  # PAPER
    if outcome == "Y":  # DRAW
        if opponent == "A":  # ROCK
            return 3 + POINTS["ROCK"]
        if opponent == "B":  # PAPER
            return 3 + POINTS["PAPER"]
        return 3 + POINTS["SCISSORS"]
    # WIN
    if opponent == "A":  # ROCK
        return 6 + POINTS["PAPER"]
    if opponent == "B":  # PAPER
        return 6 + POINTS["SCISSORS"]
    return 6 + POINTS["ROCK"]


def first_question(input_data: str) -> int:
    return sum(map(lambda x: play_round(*x.split()), input_data.splitlines()))


def second_question(input_data: str) -> int:
    return sum(map(lambda x: play_round2(*x.split()), input_data.splitlines()))


if __name__ == "__main__":

    input_data = (Path(__file__).parent / "INPUT").read_text("UTF8")
    print(first_question(input_data))
    print(second_question(input_data))
