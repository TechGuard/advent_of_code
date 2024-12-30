EXAMPLE_INPUT = '''3   4
4   3
2   5
1   3
3   9
3   3
'''

def parse_input(input):
    left = [int(x) for x in input.split()[::2]]
    right = [int(x) for x in input.split()[1::2]]
    left.sort()
    right.sort()
    return (left, right)

def part_1(input):
    left, right = parse_input(input)
    return sum(abs(left[i] - right[i]) for i in range(len(left)))


def part_2(input):
    left, right = parse_input(input)
    return sum(value * right.count(value) for value in left)