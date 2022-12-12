from pathlib import Path
from typing import List, Callable
import logging
import math

logging.basicConfig()
log = logging.getLogger(__name__)
log.setLevel(level=logging.DEBUG)


class Monkey:
    def __init__(
        self,
        items: List[int],
        operation: Callable[[int], int],
        test_divider: int,
        true_monkey: int,
        false_monkey: int,
        monkeys_list: List["Monkey"],
        magic_divider=None,
        worry_relief=3,
    ) -> None:
        self.items = items
        self.operation = operation
        self.test_divider = test_divider
        self.true_monkey = true_monkey
        self.false_monkey = false_monkey
        self.monkeys_list = monkeys_list
        self.worry_relief = worry_relief
        self.magic_divider = magic_divider
        self.inspected_items = 0

    def play(self):
        for item in self.items:
            if self.magic_divider:
                item = item % self.magic_divider
            worry = self.operation(item) // self.worry_relief
            if worry % self.test_divider == 0:
                self.monkeys_list[self.true_monkey].items.append(worry)
            else:
                self.monkeys_list[self.false_monkey].items.append(worry)
        self.inspected_items += len(self.items)
        self.items = []

    def __str__(self):
        return f"items: {self.items}\ninspected_items: {self.inspected_items}"


def first_question(input_data: str) -> int:
    monkeys_list = []

    for line in input_data.splitlines():
        line = line.strip()
        if line.startswith("Monkey") or not line:
            continue
        if line.startswith("Starting"):
            items = list(map(int, line.split(":")[1].split(",")))
            continue
        if line.startswith("Operation"):
            operation = eval(f'lambda old : {line.split("=")[1]}')
            continue
        if line.startswith("Test"):
            test_divider = int(line.split()[-1])
            continue
        if line.startswith("If true"):
            true_monkey = int(line.split()[-1])
        if line.startswith("If false"):
            false_monkey = int(line.split()[-1])
            monkeys_list.append(
                Monkey(
                    items=items,
                    operation=operation,
                    test_divider=test_divider,
                    true_monkey=true_monkey,
                    false_monkey=false_monkey,
                    monkeys_list=monkeys_list,
                )
            )

    for _ in range(20):
        for m in monkeys_list:
            m.play()

    return math.prod(sorted([x.inspected_items for x in monkeys_list], reverse=True)[:2])


def second_question(input_data: str) -> int:
    monkeys_list = []

    for line in input_data.splitlines():
        line = line.strip()
        if line.startswith("Monkey") or not line:
            continue
        if line.startswith("Starting"):
            items = list(map(int, line.split(":")[1].split(",")))
            continue
        if line.startswith("Operation"):
            operation = eval(f'lambda old : {line.split("=")[1]}')
            continue
        if line.startswith("Test"):
            test_divider = int(line.split()[-1])
            continue
        if line.startswith("If true"):
            true_monkey = int(line.split()[-1])
        if line.startswith("If false"):
            false_monkey = int(line.split()[-1])
            monkeys_list.append(
                Monkey(
                    items=items,
                    operation=operation,
                    test_divider=test_divider,
                    true_monkey=true_monkey,
                    false_monkey=false_monkey,
                    monkeys_list=monkeys_list,
                    worry_relief=1,
                )
            )

    magic_divider = math.prod([m.test_divider for m in monkeys_list])
    for m in monkeys_list:
        m.magic_divider = magic_divider

    for _ in range(10_000):
        for m in monkeys_list:
            m.play()

    return math.prod(sorted([x.inspected_items for x in monkeys_list], reverse=True)[:2])


if __name__ == "__main__":
    input_data = (Path(__file__).parent / "INPUT").read_text("UTF8")
    print(first_question(input_data))
    print(second_question(input_data))
