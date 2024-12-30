EXAMPLE_INPUT = '''rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7'''


def hash(input):
    value = 0
    for c in input.strip():
        value = (value + ord(c)) * 17 % 256
    return value


def part_1(input):
    result = 0
    for instruction in input.strip().split(','):
        result += hash(instruction)
    return result


def part_2(input):
    boxes = [[] for _ in range(256)]

    for instruction in input.strip().split(','):
        if '-' in instruction:
            label = instruction[:-1]
            n = hash(label)
            boxes[n] = [x for x in boxes[n] if x[0] != label]
        else:
            label, focalLength = instruction.split('=')
            n = hash(label)
            add = True
            for i, x in enumerate(boxes[n]):
                if x[0] == label:
                    boxes[n][i] = (label, focalLength)
                    add = False
            if add:
                boxes[n].append((label, focalLength))

    result = 0
    for n, box in enumerate(boxes):
        for slot, (_, focalLength) in enumerate(box):
            result += (n + 1) * (slot + 1) * int(focalLength)

    return result