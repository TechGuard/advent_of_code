EXAMPLE_INPUT = '''xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))'''

import re


def part_1(input):
    result = 0
    for x, y in re.findall(r'mul\(([0-9]+),([0-9]+)\)', input):
        result += int(x) * int(y)
    return result


def part_2(input):
    result = 0
    enabled = True
    for x, y, do, dont in re.findall(r'mul\(([0-9]+),([0-9]+)\)|(do\(\))|(don\'t\(\))', input):
        if do:
            enabled = True
        elif dont:
            enabled = False
        elif enabled:
            result += int(x) * int(y)
    return result