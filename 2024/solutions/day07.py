EXAMPLE_INPUT = '''190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
'''

import itertools


def solve(input, part):
    result = 0

    for line in input.splitlines():
        testval, numbers = line.split(': ')
        testval = int(testval)
        numbers = list(map(int, numbers.split()))

        options = len(numbers) - 1
        for operators in itertools.product('+*' + ('|' if part == 2 else ''), repeat=options):

            total = numbers[0]
            for i in range(1, len(numbers)):
                if operators[i-1] == '|':
                    total = int(str(total) + str(numbers[i]))
                elif operators[i-1] == '*':
                    total *= numbers[i]
                elif operators[i-1] == '+':
                    total += numbers[i]
                else:
                    assert(False)

            if total == testval:
                result += testval
                break
            
    return result


def part_1(input):
    return solve(input, 1)


def part_2(input):
    return solve(input, 2)