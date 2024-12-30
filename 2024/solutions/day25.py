EXAMPLE_INPUT = '''\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
'''


def part_1(input):
    locks, keys = [], []
    for schematic in input.split('\n\n'):
        schematic = schematic.splitlines()
        search = schematic[0][0]
        dest = locks if search == '#' else keys
        values = []
        for x in range(len(schematic[0])):
            for y in range(len(schematic)):
                if schematic[y][x] != search:
                    break
            values.append(y)
        dest.append(values)
    
    result = 0
    for lock in locks:
        for key in keys:
            if all([lhs <= rhs for lhs, rhs in zip(lock, key)]):
                result += 1

    return result


def part_2(input):
    return