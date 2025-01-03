EXAMPLE_INPUT = '''\
Time:      7  15   30
Distance:  9  40  200
'''

import math

def part_1(input):
    times, distances = [list(map(lambda x: int(x), line.split(':')[1].split())) for line in input.splitlines()]
    results = []
    for race in range(len(times)):
        time = times[race]
        distance = distances[race]
        wins = []
        for h in range(time + 1):
            speed = h / 1
            length = time - h
            dist = speed * length
            if dist > distance:
                wins.append(dist)
        results.append(len(wins))
    return math.prod(results)


def part_2(input):
    time, distance = [int(line.split(':')[1].replace(' ', '')) for line in input.splitlines()]
    result = 0
    for h in range(time + 1):
        speed = h / 1
        length = time - h
        dist = speed * length
        if dist > distance:
            result += 1
    return result