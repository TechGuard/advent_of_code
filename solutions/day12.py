EXAMPLE_INPUT = '''???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
'''

import re

def solve(springs, sizes):
    def validate(lsprings):
        last = '.'
        count = 0
        test_sizes = sizes.copy()
        for c in lsprings + ['.']:
            if c == '#':
                if last == '.':
                    count = 1
                else:
                    count += 1
            if c == '.':
                if last == '#':
                    if not test_sizes or count != test_sizes[0]:
                        return False
                    test_sizes = test_sizes[1:]
            last = c
        if test_sizes:
            return False
        return True

    # bruteforce
    def f(_lsprings, matches, cb):
        lsprings = _lsprings.copy()
        arrangements = 0
        if len(matches):
            for c in ['#', '.']:
                m = matches[0]
                lsprings[m.start()] = c
                arrangements += f(lsprings, matches[1:], cb)
        else:
            if cb(lsprings):
                arrangements += 1
        return arrangements

    matches = list(re.finditer(r'\?', springs))
    return f(list(springs), matches, validate)


def part_1(input):
    result = 0
    for line in input.splitlines():
        springs, sizes = line.split(' ')
        sizes = [int(x) for x in sizes.split(',')]
        result += solve(springs, sizes)
        
    return result


def part_2(input):
    result = 0
    for line in input.splitlines():
        springs, sizes = line.split(' ')
        sizes = [int(x) for x in sizes.split(',')]

        springs = (springs+'?') * 4 + springs
        sizes = sizes * 5

        result += solve(springs, sizes)

    return result