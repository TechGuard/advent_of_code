EXAMPLE_INPUT = '''\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
'''

DIR = {'>': (0,1), 'v': (1,0), '<': (0,-1), '^': (-1,0)}


def part_1(input):
    input_map, input_moves = input.split('\n\n')
    map = [list(line) for line in input_map.splitlines()]
    moves = input_moves.replace('\n', '')

    for y, line in enumerate(map):
        if '@' in line:
            pos = (y, line.index('@'))
            line[pos[1]] = '.'
    
    for move in moves:
        y, x = pos
        off_y, off_x = DIR[move]
        new_y = y + off_y
        new_x = x + off_x

        if map[new_y][new_x] != '.':
            # find next empty spot
            i = 0
            while True:
                c = map[new_y + off_y * i][new_x + off_x * i]
                if c == '#':
                    i = 0
                    break
                if c == '.':
                    break
                i += 1

            # invalid move
            if i == 0:
                continue

            # swap empty spot
            map[new_y + off_y * i][new_x + off_x * i] = 'O'
            map[new_y][new_x] = '.'

        pos = (new_y, new_x)

    return sum([100 * y + x for y in range(len(map)) for x in range(len(map[0])) if map[y][x] == 'O'])


def part_2(input):
    input_map, input_moves = input.split('\n\n')
    moves = input_moves.replace('\n', '')
    map = []

    for y, line in enumerate(input_map.splitlines()):
        row = ''
        for x, c in enumerate(line):
            if c == '#':
                row += '##'
            elif c == 'O':
                row += '[]'
            elif c == '.':
                row += '..'
            elif c == '@':
                pos = (y, x * 2)
                row += '..'
        map.append(list(row))
    
    def update_map(y, x, off_y, off_x, dry_run = False):
        if map[y][x] == '.':
            return True
        elif map[y][x] == '#':
            return False
        # horizontal
        elif off_y == 0:
            if not update_map(y, x + off_x, off_y, off_x):
                return False
            map[y][x + off_x] = map[y][x]
            map[y][x] = '.'
            return True
        # vertical, check if its possible first
        off_left = 0 if map[y][x] == '[' else -1
        off_right = off_left + 1
        move_left = update_map(y + off_y, x + off_left, off_y, off_x, dry_run)
        move_right = update_map(y + off_y, x + off_right, off_y, off_x, dry_run)
        if not move_left or not move_right:
            return False
        # if possible and not a dry run update the map
        if not dry_run:
            map[y + off_y][x + off_left] = map[y][x + off_left]
            map[y + off_y][x + off_right] = map[y][x + off_right]
            map[y][x + off_left] = '.'
            map[y][x + off_right] = '.'
        return True
    
    for move in moves:
        y, x = pos
        off_y, off_x = DIR[move]
        new_y = y + off_y
        new_x = x + off_x

        # do a dry_run for vertical moves
        if off_x == 0 and not update_map(new_y, new_x, off_y, off_x, True):
            continue

        if update_map(new_y, new_x, off_y, off_x):
            pos = (new_y, new_x)
            
    return sum([100 * y + x for y in range(len(map)) for x in range(len(map[0])) if map[y][x] == '['])