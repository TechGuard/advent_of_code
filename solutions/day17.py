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

from heapq import heapify, heappop, heappush

DIR = {'^': (-1, 0), '>': (0, 1), 'v': (1, 0), '<': (0, -1)}
DIR_OPPOSITE = {'^': 'v', '>': '<', 'v': '^', '<': '>'}

def solve(input, min, max):
    input = input.splitlines()
    height = len(input)
    width = len(input[0])
    
    queue = [(0, '>', 1, (0, 0)), (0, 'v', 1, (0, 0))]
    heapify(queue)
    visited = {}

    while queue:
        current = heappop(queue)
        (cDist, cDir, cDirLength, pos) = current
        for dir in '^>v<':
            if cDirLength < min and cDir != dir:
                continue
            if cDir == dir and cDirLength == max:
                continue
            if DIR_OPPOSITE[cDir] == dir:
                continue
            (y, x) = pos
            (offy, offx) = DIR[dir]
            y += offy
            x += offx
            if y < 0 or y >= height or x < 0 or x >= width:
                continue
            nDist = cDist + int(input[y][x])
            dirLength = 1 if dir != cDir else cDirLength + 1
            next = (dir, dirLength, (y, x))
            if next in visited:
                continue
            visited[next] = current
            if (y, x) == (height-1, width-1) and dirLength >= min:
                return (nDist, *next), visited
            heappush(queue, (nDist, *next))


def print_map(result, visited):
    size = max(max([y for (_,_,(y,_)) in visited]), max([x for (_,_,(_,x)) in visited])) + 1
    lines = [['.' for _ in range(size)] for _ in range(size)]

    while result:
        (_, dir, length, (y, x)) = result
        lines[y][x] = dir
        result = visited.get((dir, length, (y, x)))

    print()
    for line in lines:
        print(''.join(line))


def part_1(input):
    result, visited = solve(input, 1, 3)
    print_map(result, visited)
    return result[0]


def part_2(input):
    result, visited = solve(input, 4, 10)
    print_map(result, visited)
    return result[0]