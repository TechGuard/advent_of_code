EXAMPLE_INPUT = '''\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
'''


def find_reflection(data, tolerance):
    #   x = middle
    for x in range(len(data)):
        offset = 0
        found = False
        hitToleranceOnce = False
        while True:
            x0 = x-offset
            x1 = x+offset+1
            if x0 < 0 or x1 >= len(data): break
            # compare
            differences = sum(data[x0][i] != data[x1][i] for i in range(len(data[x0])))
            if differences == tolerance:
                hitToleranceOnce = True
            if differences <= tolerance:
                found = True
            else:
                found = False
                break
            offset += 1
        if found and hitToleranceOnce:
            return x+1


def solve(input, tolerance):
    result = 0
    for pattern in input.split('\n\n'):
        rows = [list(x) for x in pattern.splitlines()]
        columns = list(zip(*rows))

        nrows = find_reflection(rows, tolerance)
        if nrows:
            result += nrows * 100    
        else:
            ncolumns = find_reflection(columns, tolerance)
            result += ncolumns
        
    return result


def part_1(input):
    return solve(input, 0)


def part_2(input):
    return solve(input, 1)