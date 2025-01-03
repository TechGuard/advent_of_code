EXAMPLE_INPUT = '''\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
'''

class Node:
    def __init__(self, pos, type):
        self.pos = pos
        self.type = type
        self.neighbors = []
        self.visited = False

    def connect(self, nodes):
        for neighbor in self.get_neighbors():
            if neighbor in nodes:
                if self.pos in nodes[neighbor].get_neighbors():
                    self.neighbors.append(nodes[neighbor])

    def get_neighbors(self):
        (y,x) = self.pos
        if self.type == '|': return [(y-1,x),(y+1,x)]
        if self.type == '-': return [(y,x-1),(y,x+1)]
        if self.type == 'L': return [(y-1,x),(y,x+1)]
        if self.type == 'J': return [(y-1,x),(y,x-1)]
        if self.type == '7': return [(y+1,x),(y,x-1)]
        if self.type == 'F': return [(y+1,x),(y,x+1)]
        if self.type == 'S': return [(y-1,x),(y,x+1),(y+1,x),(y,x-1)]
        return []

    def next(self):
        for neighbor in self.neighbors:
            if neighbor.visit():
                return neighbor
        return None
    
    def visit(self):
        if self.visited:
            return False
        self.visited = True
        return self.visited

def process_input(input):
    nodes = {}
    for y, line in enumerate(input.splitlines()):
        for x, c in enumerate(line):
            if c != '.':
                nodes[(y,x)] = Node((y,x), c)
            if c == 'S':
                start = nodes[(y,x)]
    for node in nodes.values():
        node.connect(nodes)
    return start

def part_1(input):
    start = process_input(input)

    dist = -1
    process = [start]
    while process:
        next_process = []
        for node in process:
            for neighbor in node.neighbors:
                if neighbor.visit():
                    next_process.append(neighbor)
        process = next_process
        dist += 1
    return dist


# https://math.stackexchange.com/questions/960686/properties-of-area-of-simple-polygon-with-integer-coordinates
def part_2(input):
    start = process_input(input)

    node = start
    node.visit()
    total_points = 1
    edge_points = [node.pos]
    while node:
        node = node.next()
        total_points += 1
        if node and node.type not in ['|', '-']:
            edge_points.append(node.pos)
    
    def area_by_shoelace(x, y):
        return abs( sum(i * j for i, j in zip(x,             y[1:] + y[:1]))
                   -sum(i * j for i, j in zip(x[1:] + x[:1], y            ))) / 2
    area = area_by_shoelace(*zip(*edge_points))

    # Pick's theorem
    return int((area - (total_points/2-1)) + 0.5)


# Keeping this for for your ejoyment to see my madness
# def part_2(input):
#     start, nodes = get_input(input)
#     height = len(input.splitlines())
#     width = len(input.splitlines()[0])
    
#     result = 0
#     visited = set()

#     def fill(p0, p1, dir):
#         nonlocal result
#         pmin = (min(p0[0],p1[0]), min(p0[1],p1[1]))
#         pmax = (max(p0[0],p1[0]), max(p0[1],p1[1]))
#         if dir[0] == 0: # horizontal
#             for y in range(pmin[0], pmax[0]):
#                 x = p0[1]
#                 while True:
#                     x += dir[1]
#                     pos = (y,x)
#                     if pos in nodes:
#                         break
#                     if x > width or x < 0:
#                         break
#                     if pos not in visited:
#                         visited.add(pos)
#                         result += 1
#                         print(pos)
#         elif dir[1] == 0: # vertical
#             for x in range(pmin[1], pmax[1]):
#                 y = p0[0]
#                 while True:
#                     y += dir[0]
#                     pos = (y,x)
#                     if pos in nodes:
#                         break
#                     if y > height or y < 0:
#                         break
#                     if pos not in visited:
#                         visited.add(pos)
#                         result += 1
#                         print(pos)

#     cur_node = start
#     next_node = start.neighbors[0]
#     cur_node.visit()
#     next_node.visit()
#     dir = (next_node.pos[0]-cur_node.pos[0], next_node.pos[1]-cur_node.pos[1])
#     fill_dir = None

#     thinking = True
#     while thinking:
#         thinking = False
#         for neighbor in next_node.neighbors:
#             if neighbor.visit():
#                 thinking = True
#                 next_dir = (neighbor.pos[0]-next_node.pos[0], neighbor.pos[1]-next_node.pos[1])
#                 if dir == next_dir:
#                     next_node = neighbor
#                 else:
#                     if fill_dir is None:
#                         fill_dir = next_dir 
#                     fill(cur_node.pos, next_node.pos, fill_dir)
#                     # rotate
#                     dot = dir[0]*-next_dir[1]+dir[1]*next_dir[0]
#                     if dot > 0:
#                         fill_dir = (fill_dir[1], -fill_dir[0])
#                         print('right')
#                     else:
#                         fill_dir = (-fill_dir[1], fill_dir[0])
#                         print('left')
#                     print(fill_dir)
                    
#                     cur_node = next_node
#                     next_node = neighbor
#                     dir = next_dir
#                 break
#     return result