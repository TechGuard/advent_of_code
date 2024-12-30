EXAMPLE_INPUT = '''...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
'''

DIR = {'^': (-1, 0), '>': (0, 1), 'v': (1, 0), '<': (0, -1)}

def parse_input(input):
    map = [list(line) for line in input.splitlines()]
    size = len(map)
    
    for y in range(size):
        for x in range(size):
            if map[y][x] == 'S':
                map[y][x] = '.'
                return map, size, (y, x)


def solve_cell(map, size, start, steps, show=False):
    positions = set([start])
    for _ in range(steps):
        next_positions = set()
        for pos in positions:
            for dir in '^>v<':
                (y, x) = pos
                (offy, offx) = DIR[dir]
                y += offy
                x += offx
                if y < 0 or y >= size or x < 0 or x >= size or map[y][x] != '.':
                    continue
                next_positions.add((y, x))
        positions = next_positions

    # for debugging, inspect each cell
    if show:
        print()
        lines = [list('.'*size) for _ in range(size)]
        for (y, x) in positions:
            lines[y][x] = 'O'
        print('\n'.join([''.join(line) for line in lines]))

    return len(positions)


def part_1(input):
    map, size, start = parse_input(input)
    return solve_cell(map, size, start, 64)


def part_2(input):
    # because the input repeats there are only a few unique cells
    # filled even / odd
    # 4 edges big / small
    # 4 corners
    # https://raw.githubusercontent.com/democat3457/AdventOfCode/master/2023/resources/day21gridvis.png
    map, size, start = parse_input(input)
    grid_size = (26501365 - size // 2) // size
    result = 0

    # filled
    filled_even_count = solve_cell(map, size, start, size * 2)
    filled_even_cells = grid_size ** 2
    result += filled_even_count * filled_even_cells

    filled_odd_count = solve_cell(map, size, start, size * 2 + 1)
    filled_odd_cells = (grid_size - 1) ** 2
    result += filled_odd_count * filled_odd_cells
    
    # edges
    small_edge_count = 0
    big_edge_count = 0
    for pos in [(0, 0), (0, size - 1), (size - 1, 0), (size - 1, size - 1)]:
        small_edge_count += solve_cell(map, size, pos, size // 2 - 1)
        big_edge_count += solve_cell(map, size, pos, size * 3 // 2 - 1)

    result += small_edge_count * grid_size
    result += big_edge_count * (grid_size - 1)

    # corners
    for pos in [(start[0], 0), (start[0], size - 1), (0, start[1]), (size - 1, start[1])]:
        result += solve_cell(map, size, pos, size - 1)

    return result