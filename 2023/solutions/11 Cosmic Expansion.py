EXAMPLE_INPUT = '''\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
'''

import itertools

def process_input(input, expand):
    input = input.splitlines()
    galaxies = []
    y = 0
    for line in input:
        found_galaxy = False
        x = 0
        for lx, c in enumerate(line):
            if c == '#':
                galaxies.append((len(galaxies), y, x))
                found_galaxy = True
            if len([1 for l in input if l[lx] == '.']) == len(input):
               x += expand-1
            x += 1
        if not found_galaxy:
            y += expand-1
        y += 1
    return galaxies

def solve(galaxies):
    result = 0
    for pair in itertools.combinations(galaxies, 2):
        minp = (min(pair[0][1], pair[1][1]), min(pair[0][2], pair[1][2]))
        maxp = (max(pair[0][1], pair[1][1]), max(pair[0][2], pair[1][2]))
        dist = (maxp[0] - minp[0]) + (maxp[1] - minp[1])
        result += dist
    return result

def part_1(input):
    galaxies = process_input(input, 2)
    return solve(galaxies)

def part_2(input):
    galaxies = process_input(input, 1000000)
    return solve(galaxies)