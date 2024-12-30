EXAMPLE_INPUT = '''p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
'''


def parse_input(input):
    if input == EXAMPLE_INPUT:
        WIDTH = 11
        HEIGHT = 7
    else:
        WIDTH = 101
        HEIGHT = 103
    robots = [
        [list(map(int, part[2:].split(','))) for part in line.split()]
        for line in input.splitlines()
    ]
    return (robots, WIDTH, HEIGHT)


def part_1(input):
    robots, WIDTH, HEIGHT = parse_input(input)

    for _ in range(100):
        for pos, vel in robots:
            pos[0] = (pos[0] + vel[0]) % WIDTH
            pos[1] = (pos[1] + vel[1]) % HEIGHT

    quads = [0, 0, 0, 0]
    for pos, _ in robots:
        if pos[0] != WIDTH // 2 and pos[1] != HEIGHT // 2:
            quad_x = pos[0] // ((WIDTH + 1) // 2)
            quad_y = pos[1] // ((HEIGHT + 1) // 2)
            quads[quad_x + quad_y * 2] += 1

    result = 1
    for num in quads:
        result *= num
    return result


def part_2(input):
    robots, WIDTH, HEIGHT = parse_input(input)

    seconds = 0
    while True:
        seconds += 1

        # divide map into 64 cells
        CELL_C = 8
        cells = [0 for _ in range(CELL_C * CELL_C)]

        for pos, vel in robots:
            pos[0] = (pos[0] + vel[0]) % WIDTH
            pos[1] = (pos[1] + vel[1]) % HEIGHT

            cell_x = pos[0] // ((WIDTH + CELL_C - 1) // CELL_C)
            cell_y = pos[1] // ((HEIGHT + CELL_C - 1) // CELL_C)
            cells[cell_x + cell_y * CELL_C] += 1

        # break if one cell has over (arbitrary) 64 robots
        # most likely they formed a pattern if they're all near each other.
        cells.sort(reverse=True)
        if cells[0] > 64:
            break
        
    return seconds