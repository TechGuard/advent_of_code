EXAMPLE_INPUT = '''AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
'''

from collections import defaultdict

DIR = [(0,1), (1,0), (0,-1), (-1,0)]


def solve_regions(input):
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
                        edges.append((new_y, new_x))
                        continue
                    
                    # continue mapping of current type
                    stack.append((new_y, new_x, input[new_y][new_x]))

            regions.append((input[start_y][start_x], area, edges))

    return regions


def part_1(input):
    regions = solve_regions(input)
    return sum(area * len(edges) for _, area, edges in regions)


def part_2(input):
    region_sides = []
    regions = solve_regions(input)

    # for typ, area, edges in regions:
    #     length = len(input.splitlines()[0]) + 2
    #     lines = [['.'] * length for _ in range(length)]

    #     for y,x in edges:
    #         lines[y][x] = typ

    #     for line in lines:
    #         print(''.join(line))

    # count the corners
    # for typ, area, edges in regions:
    #     length = len(input.splitlines()[0]) + 2
    #     lines = [['.'] * length for _ in range(length)]

    #     for y,x in edges:
    #         lines[y][x] = typ

    #     corners = 0
    #     for y, x in set(edges):
    #         # convex corners
    #         if (y, x-1) not in edges and (y+1, x-1) in edges:
    #             corners += 1
    #             lines[y][x] = 'o'
    #         if (y, x+1) not in edges and (y+1, x+1) in edges:
    #             corners += 1
    #             lines[y][x] = 'o'
    #         # concave corner (nothing diagonal)
    #         if (y+1, x+1) not in edges and (y+1, x-1) not in edges and (y-1, x-1) not in edges and (y-1, x+1) not in edges:
    #             # nothing above/below
    #             if (y+1, x) not in edges and (y-1, x) not in edges:
    #                 if (y, x-1) not in edges and (y+1, x) not in edges:
    #                     corners += 1
    #                 if (y, x-1) not in edges and (y-1, x) not in edges:
    #                     corners += 1
    #             # nothing left/right
    #             if (y, x+1) not in edges and (y, x-1) not in edges:
    #                 if (y-1, x) not in edges and (y, x+1) not in edges:
    #                     corners += 1
    #                 if (y-1, x) not in edges and (y, x-1) not in edges:
    #                     corners += 1

    #     for line in lines:
    #         print(''.join(line))

    #     # amount of corners equals the amount of sides
    #     print(typ, area, corners)
    #     region_sides.append(corners)

    # for typ, area, _, corners in regions:
    #     sides = 1
    #     dir = 0
    #     pos = tiles[0]
    #     while pos != tiles[0] or dir != 3:
    #         # move forward
    #         off_y, off_x = DIR[dir]
    #         new_pos = (pos[0] + off_y, pos[1] + off_x)

    #         # check left
    #         left_dir = (dir + 3) % len(DIR)
    #         off_y, off_x = DIR[left_dir]
    #         left_pos = (new_pos[0] + off_y, new_pos[1] + off_x)
    #         if left_pos in tiles:
    #             # found corner turn left
    #             dir = left_dir
    #             sides += 1
    #             pos = new_pos
    #             continue

    #         # found corner turn right
    #         if new_pos not in tiles:
    #             dir = (dir + 1) % len(DIR)
    #             sides += 1
    #             continue

    #         # continue straight
    #         pos = new_pos

    #     region_sides.append(sides)

    # # find regions that are contained inside others and add their sides
    # for i in range(len(regions)):
    #     for j in range(len(regions)):
    #         if i == j:
    #             continue
    #         tiles_i = regions[i][3]
    #         tiles_j = regions[j][3]
    #         # aabb check
    #         (x1_i, y1_i), (x2_i, y2_i) = (min(tiles_i), max(tiles_i))
    #         (x1_j, y1_j), (x2_j, y2_j) = (min(tiles_j), max(tiles_j))
    #         if x1_i < x1_j < x2_i and x1_i < x2_j < x2_i and y1_i < y1_j < y2_i and y1_i < y2_j < y2_i:
    #             print(regions[j][0], 'inside', regions[i][0])
    #             region_sides[i] += region_sides[j]
            
    # 918626 too high
    # 348740 too low
    # return sum(regions[i][1] * region_sides[i] for i in range(len(regions)))
    