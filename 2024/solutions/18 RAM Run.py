EXAMPLE_INPUT = '''\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
'''

from heapq import heapify, heappop, heappush

DIR = [(0,1), (1,0), (0,-1), (-1,0)]


def parse_input(input):
    if input == EXAMPLE_INPUT:
        SIZE = 6 + 1
    else:
        SIZE = 70 + 1

    start = (1, 1)
    end = (SIZE, SIZE)
    bytes = [
        tuple([int(x) + 1 for x in reversed(line.split(','))])
        for line in input.splitlines()
    ]

    # build grid with padding
    grid = [list('#' + '.' * SIZE + '#') for _ in range(SIZE)]
    grid = [['#'] * len(grid[0])] + grid + [['#'] * len(grid[0])]

    return (grid, bytes, start, end)


def find_best_path(grid, start, end, start_path = []):
    queue = [(0, start_path, start)]
    heapify(queue)
    visited = set()

    while queue:
        current = heappop(queue)
        steps, path, pos = current
        if pos in visited:
            continue

        visited.add(pos)
        path = path + [pos]

        if pos == end:
            return path

        for off_y, off_x in DIR:
            y, x = pos[0] + off_y, pos[1] + off_x
            if grid[y][x] != '#':
                heappush(queue, (steps + 1, path, (y, x)))


def part_1(input):
    if input == EXAMPLE_INPUT:
        SIMULATE = 12
    else:
        SIMULATE = 1024

    grid, bytes, start, end = parse_input(input)
    for byte in bytes[:SIMULATE]:
        y, x = byte
        grid[y][x] = '#'

    return len(find_best_path(grid, start, end)) - 1


def part_2(input):
    grid, bytes, start, end = parse_input(input)
    best_path = find_best_path(grid, start, end)

    for byte in bytes:
        y, x = byte
        grid[y][x] = '#'

        if byte not in best_path:
            continue

        # byte is blocking path, calculate new path by walking one step backwards and continuing
        offset = best_path.index(byte) - 1
        best_path = find_best_path(grid, best_path[offset], end, best_path[:max(0, offset - 1)])
        if best_path is None:
            return '{},{}'.format(x - 1, y - 1)