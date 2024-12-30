EXAMPLE_INPUT = '''125 17
'''


def update_stone(number):
    if number == '0':
        return ['1']
    s = len(number)
    if s % 2 == 0:
        return [str(int(number[:s//2])), str(int(number[s//2:]))]
    return [str(int(number) * 2024)]


def solve(input, blinks):
    cache = {}
    def calc_stone(stone, n):
        if n == blinks:
            return 1
        idx = (stone,n)
        if idx in cache:
            return cache[idx]
        cache[idx] = sum(calc_stone(stone, n + 1) for stone in update_stone(stone))
        return cache[idx]
    return sum(calc_stone(stone, 0) for stone in input.split())


def part_1(input):
    return solve(input, 25)


def part_2(input):
    return solve(input, 75)