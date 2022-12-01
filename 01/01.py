from pathlib import Path
from typing import List


def first_question(input_data: str) -> int:
    lines = input_data.splitlines()
    calories = 0
    max_calories = 0
    for l in lines:
        if not l:
            if calories > max_calories:
                max_calories = calories
            calories = 0
        else:
            calories += int(l)
    return max_calories


def second_question(input_data: str) -> List[int]:
    lines = input_data.splitlines()
    calories = 0
    max_calories = [0, 0, 0]
    for l in lines:
        if not l:
            max_calories = sorted(max_calories)
            for i, m in enumerate(max_calories):
                if calories > m:
                    max_calories[i] = calories
                    break
            calories = 0
        else:
            calories += int(l)
    return sum(max_calories)


if __name__ == "__main__":

    input_data = (Path(__file__).parent / "INPUT").read_text("UTF8")

    print(first_question(input_data))
    print(second_question(input_data))
