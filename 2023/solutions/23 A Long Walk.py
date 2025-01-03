EXAMPLE_INPUT = '''\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
'''

DIR = {'^': (-1, 0), '>': (0, 1), 'v': (1, 0), '<': (0, -1)}

def solve(input, part1):
    input = input.splitlines()
    size = len(input)

    for x in range(size):
        if input[0][x] == '.':
            start = (0, x)
        if input[size - 1][x] == '.':
            target = (size - 1, x)

    nodes = {}
    
    # find possible routes
    process = [start]
    while process:
        pos = process.pop()

        dirs = '^>v<'
        (y, x) = pos
        if part1 and input[y][x] != '.':
            dirs = input[y][x]

        neighbors = []
        for dir in dirs:
            (y, x) = pos
            (offy, offx) = DIR[dir]
            y += offy
            x += offx
            if y < 0 or y >= size or x < 0 or x >= size or input[y][x] == '#':
                continue
            next = (y, x)
            neighbors.append(next)
            if next not in nodes:
                process.append(next)
        
        nodes[pos] = neighbors

    # optimize route and only keep crossroads
    new_nodes = {}
    for node in nodes:
        prev_neighbors = nodes[node]
        if len(prev_neighbors) != 2:
            neighbors = []
            for pos in prev_neighbors:
                d = 1
                prev = node
                while len(nodes[pos]) == 2:
                    for next in nodes[pos]:
                        if next != prev:
                            prev = pos
                            pos = next
                            d += 1
                            break
                neighbors.append((d, pos))
            new_nodes[node] = neighbors

    nodes = new_nodes

    # find longest distance
    process = [(start, 0, [])]
    result = 0

    while process:
        pos, dist, visited = process.pop()
        if pos in visited:
            continue
        visited.append(pos)
    
        for d, next in nodes[pos]:
            if next == target:
                result = max(result, dist + d)
            else:
                process.append((next, dist + d, visited[:]))

    return result


def part_1(input):
    return solve(input, True)


def part_2(input):
    return solve(input, False)