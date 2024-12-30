EXAMPLE_INPUT = '''Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
'''


def part_1(input):
    result = 0
    for line in input.splitlines():
        (lcard, rcard) = [c.split() for c in line[line.find(':')+1:].split('|')]
        points = None
        for n in lcard:
            if n in rcard:
                if points is None:
                    points = 1
                else:
                    points *= 2
        if points is not None:
            result += points
    return result


def part_2(input):
    result = 0
    winnings = {}
    for (card, line) in enumerate(input.splitlines()):
        (lcard, rcard) = [c.split() for c in line[line.find(':')+1:].split('|')]
        matches = 0
        for n in lcard:
            if n in rcard:
                matches += 1
        
        result += winnings.get(card, 1)
        for i in range(matches):
            x = card + i + 1
            if x in winnings:
                winnings[x] += winnings.get(card, 1)
            else:
                winnings[x] = winnings.get(card, 1) + 1
    return result