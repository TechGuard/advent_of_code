EXAMPLE_INPUT = '''....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
'''


def parse_input(input):
    input = [list(line) for line in input.splitlines()]
    for y in range(len(input)):
        if '^' in input[y]:
            guard = ((y,input[y].index('^')),(-1,0))
    return (input, guard)


def solve_step(input, guard):
    pos, dir = guard
    new_pos = (pos[0] + dir[0], pos[1] + dir[1])
    if new_pos[0] < 0 or new_pos[0] >= len(input) or new_pos[1] < 0 or new_pos[1] >= len(input[0]):
        return None
    elif input[new_pos[0]][new_pos[1]] != '#':
        return (new_pos,dir)
    else:
        return (pos,(dir[1], -dir[0]))


def part_1(input):
    input, guard = parse_input(input)
    
    visited = set()
    while guard:
        visited.add(guard[0])
        guard = solve_step(input, guard)
    
    return len(visited)


def part_2(input):
    input, guard = parse_input(input)
    
    def will_it_loop(guard, input):
        visited = set()
        while guard:
            if guard in visited:
                return True
            visited.add(guard)
            guard = solve_step(input, guard)
        return False
    
    result = 0
    for y in range(len(input)):
        for x in range(len(input[0])):
            if input[y][x] == '.':
                input[y][x] = '#'
                if will_it_loop(guard, input):
                    result += 1
                input[y][x] = '.'

    return result