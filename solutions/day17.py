EXAMPLE_INPUT = '''2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
'''

DIR = {'^': (-1, 0), '>': (0, 1), 'v': (1, 0), '<': (0, -1)}
DIR_OPPOSITE = {'^': 'v', '>': '<', 'v': '^', '<': '>'}

def solve(input, min, max):
    input = input.splitlines()
    height = len(input)
    width = len(input[0])
    
    queue = [(0, '>', 1, (0, 0)), (0, 'v', 1, (0, 0))]
    visited = set()

    while queue:
        queue.sort(reverse=True)
        (cDist, cDir, cDirLength, current) = queue.pop()
        if (cDir, cDirLength, current) in visited:
            continue
        visited.add((cDir, cDirLength, current))

        for dir in '^>v<':
            if cDirLength < min and cDir != dir:
                continue
            if cDir == dir and cDirLength == max:
                continue
            if DIR_OPPOSITE[cDir] == dir:
                continue
            (y, x) = current
            (offy, offx) = DIR[dir]
            y += offy
            x += offx
            if y < 0 or y >= height or x < 0 or x >= width:
                continue
            nDist = cDist + int(input[y][x])
            dirLength = 1 if dir != cDir else cDirLength + 1
            if (y, x) == (height-1, width-1) and dirLength >= min:
                return nDist
            queue.append((nDist, dir, dirLength, (y, x)))


def part_1(input):
    return solve(input, 1, 3)


def part_2(input):
    return solve(input, 4, 10)