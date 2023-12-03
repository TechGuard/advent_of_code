EXAMPLE_INPUT = '''467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
'''

def part_1(input):
    grid = input.splitlines()
    height = len(grid)
    width = len(grid[0])
    result = 0
    # for each line and character
    for y1 in range(height):
        xIter = iter(range(width))
        for x1 in xIter:
            if grid[y1][x1].isdigit():
                # x1 begin of number x2 end of number
                x2 = next(xIter, width)
                while x2 < width and grid[y1][x2].isdigit():
                    x2 = next(xIter, width)
                def find_adjecent():
                    for y in range(y1-1, min(y1+2, height)):
                        for x in range(x1-1, min(x2+1, width)):
                            if grid[y][x] != '.' and not grid[y][x].isdigit():
                                return True
                if find_adjecent():
                    result += int(grid[y1][x1:x2])
    return result


def part_2(input):
    grid = input.splitlines()
    height = len(grid)
    width = len(grid[0])
    gears = {}
    # for each line and character
    for y1 in range(height):
        xIter = iter(range(width))
        for x1 in xIter:
            if grid[y1][x1].isdigit():
                # x1 begin of number x2 end of number
                x2 = next(xIter, width)
                while x2 < width and grid[y1][x2].isdigit():
                    x2 = next(xIter, width)
                # find adjecent gears
                for y in range(y1-1, min(y1+2, height)):
                    for x in range(x1-1, min(x2+1, width)):
                        if grid[y][x] == '*':
                            if (y,x) not in gears:
                                gears[(y,x)] = []
                            gears[(y,x)].append(int(grid[y1][x1:x2]))
    # count wrong gears
    result = 0
    for values in gears.values():
        if len(values) > 1:
            ratio = 1
            for value in values: ratio *= value
            result += ratio
    return result