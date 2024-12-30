EXAMPLE_INPUT = '''7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
'''


def is_safe(levels):
    last_dir = None
    for i in range(len(levels) - 1):
        diff = levels[i + 1] - levels[i]
        dir = -1 if diff < 0 else 1
        if diff == 0 or abs(diff) > 3 or (last_dir and dir != last_dir):
            return False
        last_dir = dir
    return True


def solve(input, part):
    result = 0
    for line in input.splitlines():
        levels = [int(x) for x in line.split()]
        if is_safe(levels):
            result += 1
        elif part == 2:
            for i in range(len(levels)):
                retry = levels.copy()
                del retry[i]
                if is_safe(retry):
                    result += 1
                    break
    return result


def part_1(input):
    return solve(input, 1)


def part_2(input):
    return solve(input, 2)