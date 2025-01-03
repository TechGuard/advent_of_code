EXAMPLE_INPUT = '''\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
'''

from collections import defaultdict

DIR = [(0,1), (1,0), (0,-1), (-1,0)]


def solve(input, part):
    input = [list(map(int, line)) for line in input.splitlines()]
    peaks = [
        (y, x)
        for y, line in enumerate(input)
        for x, c in enumerate(line) if c == 9
    ]
    
    trailheads = defaultdict(set)
    unique_trails = defaultdict(list)
    for peak in peaks:
        y, x = peak
        
        stack = [([(y, x)], y, x)]
        while stack:
            trail, y, x = stack.pop()
            for next_dir in range(len(DIR)):
                off_y, off_x = DIR[next_dir]
                new_y = y + off_y
                new_x = x + off_x
                if new_y < 0 or new_y >= len(input) or new_x < 0 or new_x >= len(input):
                    continue
                if input[y][x] - input[new_y][new_x] != 1:
                    continue
                
                new_trail = trail + [(new_y, new_x)]
                if input[new_y][new_x] == 0:
                    trailheads[(new_y, new_x)].add(peak)
                    unique_trails[(new_y, new_x)].append(new_trail)
                else:
                    stack.append((new_trail, new_y, new_x))

    if part == 1:
        return sum([len(peaks) for peaks in trailheads.values()])
    else:
        return sum([len(trails) for trails in unique_trails.values()])


def part_1(input):
    return solve(input, 1)


def part_2(input):
    return solve(input, 2)