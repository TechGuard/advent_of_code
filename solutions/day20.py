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
        queue = [(0, start_pos)]
        heapify(queue)
        visited = set()

        while queue:
            time, pos = heappop(queue)
            if pos in visited:
                continue
            visited.add(pos)

            # if position is a valid path, calculate the distance skipped
            if pos in path_dist:
                cheat_save = path_dist[pos] - (path_dist[start_pos] + time)
                if cheat_save >= 100:
                    result += 1

            # cannot cheat any longer
            if time == cheat_time:
                continue

            # find next cheat position
            for off_y, off_x in DIR:
                y, x = pos[0] + off_y, pos[1] + off_x
                if 0 <= y < SIZE and 0 <= x < SIZE:
                    heappush(queue, (time + 1, (y, x)))

    return result


def part_1(input):
    return solve(input, 2)


def part_2(input):
    return solve(input, 20)