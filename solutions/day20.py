EXAMPLE_INPUT = '''\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
'''

from heapq import heapify, heappop, heappush

DIR = [(0,1), (1,0), (0,-1), (-1,0)]


def find_best_path(visited, grid, start, end):
    queue = [(0, [], start)]
    heapify(queue)

    while queue:
        current = heappop(queue)
        dist, path, pos = current
        if pos in visited:
            continue

        visited[pos] = dist
        path = path + [pos]

        if pos == end:
            return path

        for off_y, off_x in DIR:
            y, x = pos[0] + off_y, pos[1] + off_x
            if grid[y][x] != '#':
                heappush(queue, (dist + 1, path, (y, x)))


def solve(input, cheat_time):
    grid = [list(line) for line in input.splitlines()]
    SIZE = len(grid)
    for y, line in enumerate(grid):
        if 'S' in line:
            start = (y, line.index('S'))
            line[start[1]] = '.'
        if 'E' in line:
            end = (y, line.index('E'))
            line[end[1]] = '.'

    result = 0
    path_dist = {}
    best_path = find_best_path(path_dist, grid, start, end)

    # for each position along the best path
    for start_pos in best_path:
        # search in a radius of cheat_time
        for off_y in range(-cheat_time, cheat_time + 1):
            for off_x in range(-cheat_time, cheat_time + 1):
                time = abs(off_y) + abs(off_x)
                if time <= cheat_time:
                    # if the cheat position is valid, calculate the distance skipped
                    pos = (start_pos[0] + off_y, start_pos[1] + off_x)
                    if pos in path_dist:
                        cheat_save = path_dist[pos] - (path_dist[start_pos] + time)
                        if cheat_save >= 100:
                            result += 1

    return result


def part_1(input):
    return solve(input, 2)


def part_2(input):
    return solve(input, 20)