EXAMPLE_INPUT = '''RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
'''

DIR = [(0,1), (1,0), (0,-1), (-1,0)]


def find_regions(input):
    # input with padding
    input = [['.'] + list(line) + ['.'] for line in input.splitlines()]
    input = [['.'] * len(input[0])] + input + [['.'] * len(input[0])]

    regions = []
    visited = set()

    # for each tile
    for start_y in range(1, len(input) - 1):
        for start_x in range(1, len(input) - 1):
            if (start_y, start_x) in visited:
                continue

            area = 0
            edges = []
            stack = [(start_y, start_x, input[start_y][start_x])]
            while stack:
                # ignore padding and already visited tiles
                y, x, typ = stack.pop()
                if typ == '.' or (y,x) in visited:
                    continue

                # increase area of type by one
                visited.add((y,x))
                area += 1

                # for each direction increase edge count if type is not the same
                for dir in range(len(DIR)):
                    off_y, off_x = DIR[dir]
                    new_y, new_x = y + off_y, x + off_x
                    if input[new_y][new_x] != typ:
                        edges.append((new_y, new_x, dir))
                        continue
                    
                    # continue mapping of current type
                    stack.append((new_y, new_x, input[new_y][new_x]))

            regions.append((input[start_y][start_x], area, edges))

    return regions


def part_1(input):
    regions = find_regions(input)
    return sum(area * len(edges) for _, area, edges in regions)


def part_2(input):
    region_sides = []
    regions = find_regions(input)

    for _, _, edges in regions:
        sides = 0
        visited = set()

        # for each edge, visit neighbours that faces same direction until
        # no more edges are connected, that counts as one side
        def visit_neighbours(edge, dir):
            off_y, off_x = DIR[dir]
            y, x, original_dir = edge
            while True:
                y, x = y + off_y, x + off_x
                new_edge = (y, x, original_dir)
                if new_edge not in edges:
                    break
                visited.add(new_edge)

        for edge in edges:
            if edge in visited:
                continue

            visited.add(edge)
            sides += 1

            dir = (edge[2] + 1) % len(DIR) # rotate clockwise
            visit_neighbours(edge, dir) # visit in same direction
            visit_neighbours(edge, (dir + 2) % len(DIR)) # reverse direction

        region_sides.append(sides)

    return sum(regions[i][1] * region_sides[i] for i in range(len(regions)))