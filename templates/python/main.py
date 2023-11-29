
def part_one(input: str) -> int | None:
    # your code here
    ...

def part_two(input: str) -> int | None:
    # your code here
    ...

def main() -> None:
    with open('../input.txt') as f:
        input = f.read()

    print(f'{part_one(input)}')
    print(f'{part_two(input)}')

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
