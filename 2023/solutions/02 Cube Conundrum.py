EXAMPLE_INPUT = '''\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
'''

import re

def part_1(input):
    result = 0
    rules = {'red': 12, 'green': 13, 'blue': 14}
    for id, line in enumerate(input.splitlines()):
        impossible = False
        for cubes in re.split(',|;', line[line.find(':')+1:]):
            split = cubes.strip().split(' ')
            count = int(split[0])
            team = split[1]
            if rules[team] < count:
                impossible = True
                break
        if not impossible:
            result += id + 1
    return result


def part_2(input):
    result = 0
    for line in input.splitlines():
        min = {}
        for cubes in re.split(',|;', line[line.find(':')+1:]):
            split = cubes.strip().split(' ')
            count = int(split[0])
            team = split[1]
            if min.get(team, 0) < count:
                min[team] = count
        power = 1
        for n in min.values(): power *= n
        result += power
    return result