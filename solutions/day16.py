EXAMPLE_INPUT = r'''.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
'''

from collections import defaultdict

def solve(input, start):
    width = len(input[0])
    height = len(input)

    energized = defaultdict(lambda: set())
    beams = [start]
    while beams:
        pos, dir = beams.pop()
        pos = (pos[0] + dir[0], pos[1] + dir[1])
        if pos[0] < 0 or pos[0] >= height:
            continue
        if pos[1] < 0 or pos[1] >= width:
            continue

        # repeating
        if dir in energized[(pos)]:
            continue
        energized[(pos)].add(dir)
        
        c = input[pos[0]][pos[1]]
        if c == '/':
            dir = (-dir[1], -dir[0])
        elif c == '\\':
            dir = (dir[1], dir[0])
        elif c == '|' and dir[1]:
            dir = (-dir[1], dir[0])
            beams.append((pos, dir))
            dir = [-x for x in dir]
        elif c == '-' and dir[0]:
            dir = (-dir[1], dir[0])
            beams.append((pos, dir))
            dir = [-x for x in dir]
        beams.append((pos, tuple(dir)))

    return len(energized.keys())


def part_1(input):
    input = input.splitlines()
    return solve(input, ((0, -1), (0, 1)))


def part_2(input):
    input = input.splitlines()
    width = len(input[0])
    height = len(input)
    result = 0
    for x in range(width):
        result = max(result, solve(input, ((-1, x), (1, 0))))
        result = max(result, solve(input, ((height, x), (-1, 0))))
    for y in range(height):
        result = max(result, solve(input, ((y, -1), (0, 1))))
        result = max(result, solve(input, ((y, width), (0, -1))))
    return result