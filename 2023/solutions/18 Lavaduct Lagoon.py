EXAMPLE_INPUT = '''\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
'''

DIRS = {'R': (0, 1), 'D': (1, 0), 'L': (0, -1), 'U': (-1, 0)}

def solve(input, parse):
    x = 0
    y = 0
    edge_points = []
    total_points = 0
    for line in input.splitlines():
        (dir, dist) = parse(line)
        (yoff, xoff) = DIRS[dir]
        y += yoff * dist
        x += xoff * dist
        edge_points.append((y,x))
        total_points += dist

    def area_by_shoelace(x, y):
        return abs( sum(i * j for i, j in zip(x,             y[1:] + y[:1]))
                   -sum(i * j for i, j in zip(x[1:] + x[:1], y            ))) / 2
    area = area_by_shoelace(*zip(*edge_points))

    return int(area + total_points / 2 + 1)


def part_1(input):
    def parse(line):
        (dir, dist, *_) = line.split()
        return (dir, int(dist))
    return solve(input, parse)


def part_2(input):
    def parse(line):
        (_, _, color) = line.split()
        dist = int(color[2:7], 16)
        dir = 'RDLU'[int(color[7])]
        return (dir, dist)
    return solve(input, parse)