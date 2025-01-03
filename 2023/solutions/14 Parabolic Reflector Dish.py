EXAMPLE_INPUT = '''\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
'''

def is_empty(rows, y, x):
    if y < 0: return False
    return rows[y][x] == '.'


def calc_load(rows):
    return sum([sum([(len(rows)-y) for x in row if x == 'O']) for y, row in enumerate(rows) if row])


def simulate(rows):
    while True:
        simulate = False
        for y, row in enumerate(rows):
            for x, c in enumerate(row):
                if c == 'O' and is_empty(rows, y-1, x):
                    simulate = True
                    rows[y-1][x] = c
                    row[x] = '.'
        if not simulate:
            break


def part_1(input):
    rows = [list(x) for x in input.splitlines()]
    simulate(rows)
    return calc_load(rows)


def part_2(input):
    rows = [list(x) for x in input.splitlines()]

    remember = {}
    stopAt = 1000000000
    for cycle in range(1000000000):
        if cycle == stopAt:
            break

        # north
        simulate(rows)

        # west
        rows = list(zip(*rows[::-1]))
        rows = [list(x) for x in rows]
        simulate(rows)

        # south
        rows = list(zip(*rows[::-1]))
        rows = [list(x) for x in rows]
        simulate(rows)

        # east
        rows = list(zip(*rows[::-1]))
        rows = [list(x) for x in rows]
        simulate(rows)

        # north
        rows = list(zip(*rows[::-1]))
        rows = [list(x) for x in rows]
        
        key = tuple([tuple(x) for x in rows])
        if key in remember:
            # cycle repeats
            remainder = (1000000000 - remember[key]) % (cycle - remember[key])
            stopAt = cycle + remainder
        remember[key] = cycle

    return calc_load(rows)