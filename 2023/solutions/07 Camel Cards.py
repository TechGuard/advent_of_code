EXAMPLE_INPUT = '''\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
'''

from functools import cmp_to_key

def get_type(hand):
    pair = False
    unique = len(set(list(hand)))
    while len(hand):
        samec = hand.count(hand[0])
        hand = hand.replace(hand[0], '')
        if samec == 5:
            return 6
        if samec == 4:
            return 5
        if samec == 3 and unique == 2:
            return 4
        if samec == 3 and unique == 3:
            return 3
        if samec == 2:
            if pair:
                return 2
            pair = True
    if pair and unique == 4:
        return 1
    return 0

def cmp_hand(ranking):
    def cmp(a, b):
        if a[0] == b[0]:
            for i in range(len(b[1])):
                x = ranking.index(b[1][i]) - ranking.index(a[1][i])
                if x != 0:
                    return x
            return 0
        return a[0] - b[0]
    return cmp_to_key(cmp)

def part_1(input):
    results = []
    for line in input.splitlines():
        hand, bid = line.split()
        results.append((get_type(hand), hand, bid))

    results.sort(key=cmp_hand(['A','K','Q','J','T','9','8','7','6','5','4','3','2']))

    result = 0
    for i, (_, _, bid) in enumerate(results):
        result += (i + 1) * int(bid)
    return result


def part_2(input):
    def get_jtype(hand):
        types = []
        for c in set(list(hand)):
            jhand = hand.replace('J', c)
            types.append(get_type(jhand))
        return max(types)
    
    results = []
    for line in input.splitlines():
        hand, bid = line.split()
        results.append((get_jtype(hand), hand, bid))

    results.sort(key=cmp_hand(['A','K','Q','T','9','8','7','6','5','4','3','2','J']))

    result = 0
    for i, (_, _, bid) in enumerate(results):
        result += (i + 1) * int(bid)
    return result