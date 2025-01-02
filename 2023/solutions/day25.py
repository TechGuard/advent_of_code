EXAMPLE_INPUT = '''jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
'''

import networkx
import itertools

def part_1(input):
    g = networkx.Graph()
    for line in input.splitlines():
        node, neighbors = line.split(': ')
        for neighbor in neighbors.split():
            g.add_edge(node, neighbor, capacity=1)

    for a, b in itertools.combinations(g.nodes, 2):
        cut, partition = networkx.minimum_cut(g, a, b)
        if cut == 3:
            return len(partition[0])*len(partition[1])