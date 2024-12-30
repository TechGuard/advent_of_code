EXAMPLE_INPUT = '''\
029A
980A
179A
456A
379A
'''

from functools import cache

def parse_keypad(input):
    keypad = {}
    for y, line in enumerate(input.splitlines()):
        for x, c in enumerate(line):
            keypad[c] = (y,x)
    return keypad

NUM_KEYPAD = parse_keypad('''\
789
456
123
 0A
''')

DIR_KEYPAD = parse_keypad('''\
 ^A
<v>
''')

DIR = {'>': (0,1), 'v': (1,0), '<': (0,-1), '^': (-1,0)}


def get_sequence_options(keypad, start, end):
    valid_positions = set([pos for button, pos in keypad.items() if button != ' '])
    seq_options = []
    stack = [([], [], keypad[start])]
    while stack:
        seq, visited, pos = stack.pop()
        if pos == keypad[end]:
            seq_options.append(tuple(seq + ['A']))
            continue
        y, x = pos
        for move, dir in DIR.items():
            new_y, new_x = y + dir[0], x + dir[1]
            new_pos = (new_y, new_x)
            if new_pos in valid_positions and new_pos not in visited:
                stack.append((seq + [move], visited + [new_pos], new_pos))
    return seq_options


@cache
def shortest_seq(seq, robot_offset, num_robots):
    if robot_offset == num_robots:
        return len(seq)
    
    if robot_offset == 0:
        keypad = NUM_KEYPAD
    else:
        keypad = DIR_KEYPAD

    prev_button = 'A'
    len_seq = 0

    for button in seq:
        options = []
        for next_seq in get_sequence_options(keypad, prev_button, button):
            options.append(shortest_seq(next_seq, robot_offset + 1, num_robots))

        len_seq += min(options)
        prev_button = button

    return len_seq


def solve(input, num_robots):
    result = 0
    for code in input.splitlines():
        len_seq = shortest_seq(code, 0, num_robots)
        result += len_seq * int(code[:-1])
    return result


def part_1(input):
    return solve(input, 3)


def part_2(input):
    return solve(input, 26)