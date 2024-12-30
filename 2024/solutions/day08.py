EXAMPLE_INPUT = '''............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
'''

import itertools


def solve(input, part):
    antenna_groups = {}
    for y, line in enumerate(input.splitlines()):
        SIZE = y+1
        for x, c in enumerate(line):
            if c != '.':
                if c in antenna_groups:
                    antenna_groups[c].append((y,x))
                else:
                    antenna_groups[c] = [(y,x)]
    
    anti_nodes = set()
    for antennas in antenna_groups.values():
        # Find each combination of antennas
        for (first, second) in itertools.combinations(antennas, 2):
            # Add anti node at antenna location
            if part == 2:
                anti_nodes.add(first)
                anti_nodes.add(second)

            # Just add a bunch of antinodes and we cull them later
            diff = (first[0] - second[0], first[1] - second[1])
            for _ in range(SIZE if part == 2 else 1):
                # calculate anti nodes
                first = (first[0] + diff[0], first[1] + diff[1])
                anti_nodes.add(first)
                second = (second[0] - diff[0], second[1] - diff[1])
                anti_nodes.add(second)

    return len([1 for (y,x) in anti_nodes if y >= 0 and y < SIZE and x >= 0 and x < SIZE])


def part_1(input):
    return solve(input, 1)


def part_2(input):
    return solve(input, 2)