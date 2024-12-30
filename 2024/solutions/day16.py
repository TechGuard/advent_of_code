EXAMPLE_INPUT = '''###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
'''

from heapq import heapify, heappop, heappush

DIR = [(0,1), (1,0), (0,-1), (-1,0)]


def solve(input):
    input = [list(line) for line in input.splitlines()]
    for y, line in enumerate(input):
        if 'S' in line:
            start = (y,line.index('S'))
            line[start[1]] = '.'
        if 'E' in line:
            end = (y,line.index('E'))
            line[end[1]] = '.'

    queue = [(0, 0, start)]
    heapify(queue)
    visited = {}
    result = None

    while queue:
        current = heappop(queue)
        score, dir, pos = current
        if (dir, pos) in visited:
            continue
        visited[(dir, pos)] = score

        if pos == end:
            result = (dir, pos)
            break

        # move forward
        off_y, off_x = DIR[dir]
        y, x = pos[0] + off_y, pos[1] + off_x
        if input[y][x] != '#':
            heappush(queue, (score + 1, dir, (y, x)))

        # turn right
        heappush(queue, (score + 1000, (dir + 1) % len(DIR), pos))
        # turn left
        heappush(queue, (score + 1000, (dir + 3) % len(DIR), pos))
    
    return visited, result


def part_1(input):
    path, end = solve(input)
    return path[end]


def part_2(input):
    path, end = solve(input)

    tiles = set()
    stack = [(path[end], *end)]
    
    while stack:
        item = stack.pop()
        score, dir, pos = item
        tiles.add(pos)

        # only add next tile if its a vald path (ie score change is what we expect)
        def add_next(next, next_score):
            if next in path and path[next] == next_score:
                stack.append((next_score, *next))

        # move backward
        reverse_dir = (dir + 2) % len(DIR)
        off_y, off_x = DIR[reverse_dir]
        y, x = pos[0] + off_y, pos[1] + off_x
        add_next((dir, (y, x)), score - 1)

        # turn right
        add_next(((dir + 1) % len(DIR), pos), score - 1000)
        # turn left
        add_next(((dir + 3) % len(DIR), pos), score - 1000)

    return len(tiles)