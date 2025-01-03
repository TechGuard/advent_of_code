EXAMPLE_INPUT = '''\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
'''

import itertools
import re

from math import lcm
from functools import reduce

def part_1(input):
    sequence, _, *instructions = input.splitlines()

    map = {}
    for instruction in instructions:
        m = re.match(r'(.+) = \((.+), (.+)\)', instruction)
        map[m.group(1)] = (m.group(2), m.group(3))

    pos = 'AAA'
    for result in itertools.count():
        if pos == 'ZZZ':
            return result
        op = sequence[result % len(sequence)]
        pos = map[pos][0 if op == 'L' else 1]


def part_2(input):
    sequence, _, *instructions = input.splitlines()

    map = {}
    nodes = []
    for instruction in instructions:
        m = re.match(r'(.+) = \((.+), (.+)\)', instruction)
        pos = m.group(1)
        map[pos] = (m.group(2), m.group(3))
        if pos.endswith('A'):
            nodes.append(pos)

    results = []
    for result in itertools.count():
        new_nodes = []
        for pos in nodes:
            if pos.endswith('Z'):
                results.append(result)
                continue
            op = sequence[result % len(sequence)]
            pos = map[pos][0 if op == 'L' else 1]
            new_nodes.append(pos)
        nodes = new_nodes
        if not nodes:
            break

    return reduce(lcm, results)