EXAMPLE_INPUT = '''\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
'''


def part_1(input):
    input = [list(line) for line in input.splitlines()]
    HEIGHT = len(input)
    WIDTH = len(input[0])
    SEARCH = list('XMAS')
    SEARCH_LEN = len(SEARCH)
    
    result = 0
    for start_y in range(HEIGHT):
        for start_x in range(WIDTH):
            # Horizontal
            valid = [True, True]
            for i in range(SEARCH_LEN):
                if start_x + i >= WIDTH or input[start_y][start_x + i] != SEARCH[i]:
                    valid[0] = False
                if start_x - i < 0 or input[start_y][start_x - i] != SEARCH[i]:
                    valid[1] = False
                if i == SEARCH_LEN - 1:
                    result += valid.count(True)
            # Vertical
            valid = [True, True]
            for i in range(SEARCH_LEN):
                if start_y + i >= HEIGHT or input[start_y + i][start_x] != SEARCH[i]:
                    valid[0] = False
                if start_y - i < 0 or input[start_y - i][start_x] != SEARCH[i]:
                    valid[1] = False
                if i == SEARCH_LEN - 1:
                    result += valid.count(True)
            # Diagonal
            valid = [True, True]
            for i in range(SEARCH_LEN):
                if start_y + i >= HEIGHT or start_x - i < 0 or input[start_y + i][start_x - i] != SEARCH[i]:
                    valid[0] = False
                if start_y - i < 0 or start_x + i >= WIDTH or input[start_y - i][start_x + i] != SEARCH[i]:
                    valid[1] = False
                if i == SEARCH_LEN - 1:
                    result += valid.count(True)
            # Diagonal
            valid = [True, True]
            for i in range(SEARCH_LEN):
                if start_y + i >= HEIGHT or start_x + i >= WIDTH or input[start_y + i][start_x + i] != SEARCH[i]:
                    valid[0] = False
                if start_y - i < 0 or start_x - i < 0 or input[start_y - i][start_x - i] != SEARCH[i]:
                    valid[1] = False
                if i == SEARCH_LEN - 1:
                    result += valid.count(True)
                
    return result


def part_2(input):
    input = [list(line) for line in input.splitlines()]
    HEIGHT = len(input)
    WIDTH = len(input[0])
    
    result = 0
    for y in range(HEIGHT - 2):
        for x in range(WIDTH - 2):
            diagonal1 = (
                input[y][x] == 'M' and input[y + 1][x + 1] == 'A' and input[y + 2][x + 2] == 'S',
                input[y][x] == 'S' and input[y + 1][x + 1] == 'A' and input[y + 2][x + 2] == 'M'
            )
            diagonal2 = (
                input[y][x + 2] == 'M' and input[y + 1][x + 1] == 'A' and input[y + 2][x] == 'S',
                input[y][x + 2] == 'S' and input[y + 1][x + 1] == 'A' and input[y + 2][x] == 'M'
            )
            if diagonal1.count(True) and diagonal2.count(True):
                result += 1
                
    return result