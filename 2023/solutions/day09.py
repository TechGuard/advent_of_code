EXAMPLE_INPUT = '''0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
'''


def process_input(input):
    sequences = []
    for line in input.splitlines():
        seq = [int(x) for x in line.split()]
        full_seq = [seq]
        while seq.count(0) != len(seq):
            next_seq = []
            prev = seq[0]
            for n in seq[1:]:
                next_seq.append(n-prev)
                prev = n
            seq = next_seq
            full_seq.append(seq)
        sequences.append(full_seq)
    return sequences


def part_1(input):
    sequences = process_input(input)
    result = 0
    for full_seq in sequences:
        val = 0
        for seq in full_seq:
            val += seq[-1]
        result += val
    return result


def part_2(input):
    sequences = process_input(input)
    result = 0
    for full_seq in sequences:
        val = 0
        for seq in reversed(full_seq):
            val = seq[0] - val
        result += val
    return result