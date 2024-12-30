EXAMPLE_INPUT = '''\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
'''

from collections import defaultdict


def find_inter_connections(input, part):
    all_connections = defaultdict(set)
    for line in input.splitlines():
        lhs, rhs = line.split('-')
        all_connections[lhs].add(rhs)
        all_connections[rhs].add(lhs)

    inter_connections = []

    # Bron-Kerbosch algorithm
    def find_largest_connections(remaining, excluded=set(), current_connections=set()):
        if part == 1 and len(current_connections) == 3 or (part == 2 and len(remaining) == 0 and len(excluded) == 0 and len(current_connections) > 2):
            return inter_connections.append(tuple(sorted(current_connections)))
        for computer in remaining.copy():
            find_largest_connections(
                remaining.intersection(all_connections[computer]),
                excluded.intersection(all_connections[computer]),
                current_connections.union([computer]),
            )
            remaining.remove(computer)
            excluded.add(computer)

    find_largest_connections(set(all_connections.keys()))
    return inter_connections


def part_1(input):
    inter_connections = find_inter_connections(input, 1)
    return len([connections for connections in inter_connections if any(computer[0] == 't' for computer in connections)])


def part_2(input):
    inter_connections = find_inter_connections(input, 2)
    inter_connections.sort(reverse=True, key=lambda connections: len(connections))
    return ','.join(inter_connections[0])