import sys

def part_one(input: str) -> int | None:
    # your code here
    ...

def part_two(input: str) -> int | None:
    # your code here
    ...

def main() -> None:
    with open('../input.txt') as f:
        input = f.read()

    match sys.argv[1]:
        case '1': print(f'{part_one(input)}')
        case '2': print(f'{part_two(input)}')
        case _ : print('Please specify a part to run (1 or 2)')

if __name__ == '__main__':
    main()

# pytest

import pytest

from main import part_one, part_two

@pytest.fixture
def input() -> str:
    with open('../example.txt') as f:
        input = f.read()
    return input

def test_part_one(input: str) -> None:
    assert part_one(input) == None # put the expected answer here

def test_part_two(input: str) -> None:
    assert part_two(input) == None # put the expected answer here
