EXAMPLE_INPUT = '''1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
'''

import copy

class Brick:
    removed = False


def simulate(_bricks, ignore=None):
    if ignore:
        bricks = copy.deepcopy(_bricks)
        for b in bricks:
            if b.name == ignore.name:
                bricks.remove(b)
                break
    else:
        bricks = _bricks

    simulate = True
    simulated = 0
    while simulate:
        simulate = False
        process = bricks.copy()
        while process:
            brick = process.pop()
            if brick == ignore:
                continue
            minZ = 1
            for other in bricks:
                if other == brick: continue
                if other == ignore: continue
                if brick.minX < other.maxX and brick.maxX > other.minX and brick.minY < other.maxY and brick.maxY > other.minY:
                    if brick.minZ >= other.maxZ:
                        minZ = max(minZ, other.maxZ)
            if minZ != brick.minZ:
                simulated += 1
                simulate = True
                diff = brick.minZ - minZ
                brick.minZ -= diff
                brick.maxZ -= diff
                
    return simulated


def print_grid(bricks):
    grid = {}
    size = [0, 0, 0]
    for brick in bricks:
        for x in range(brick.minX, brick.maxX):
            size[0] = max(x, size[0])
            for y in range(brick.minY, brick.maxY):
                size[1] = max(y, size[1])
                for z in range(brick.minZ, brick.maxZ):
                    grid[(x,y,z)] = brick
                    size[2] = max(z, size[2])

    print()
    for z in range(size[2], 0, -1):
        line = []
        for x in range(size[0]+1):
            c = '.'
            for y in range(size[1]+1):
                pos = (x,y,z)
                if pos in grid:
                    c = grid[pos].name
            line.append(c)
        print(''.join(line))


def parse_input(input):
    bricks = []
    for i, line in enumerate(input.splitlines()):
        l, r = line.split('~')
        p0 = [int(x) for x in l.split(',')]
        p1 = [int(x) for x in r.split(',')]
        brick = Brick()
        brick.name = chr(ord('A')+i)
        brick.minX = p0[0]
        brick.minY = p0[1]
        brick.minZ = p0[2]
        brick.maxX = p1[0]+1
        brick.maxY = p1[1]+1
        brick.maxZ = p1[2]+1
        bricks.append(brick)

    bricks.sort(reverse=True, key=lambda b: b.maxZ)

    simulate(bricks)
    # print_grid(bricks)

    return bricks


result_p1 = 0
result_p2 = 0

def part_1(input):
    global result_p1
    global result_p2
    bricks = parse_input(input)
    for brick in bricks:
        result = simulate(bricks, brick)
        if not result:
            result_p1 += 1
        result_p2 += result

    return result_p1


def part_2(_):
    global result_p2
    return result_p2