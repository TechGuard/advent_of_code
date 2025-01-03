EXAMPLE_INPUT = '''\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
'''


def part_1(input):
    total = 0
    for line in input.splitlines():
        first = None
        last = None
        for c in line:
            if c.isdigit():
                if first is None: first = c
                last = c
        total += int(first + last)
    return total


def part_2(input):
    total = 0
    for line in input.splitlines():
        first = None
        last = None
        for i in range(len(line)):
            c = line[i]
            if c.isdigit():
                if first is None: first = c
                last = c
            else:
                for j, x in enumerate(['one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine']):
                    if line[i:i+len(x)] == x:
                        if first is None: first = str(j+1)
                        last = str(j+1)
        total += int(first + last)
    return total
    