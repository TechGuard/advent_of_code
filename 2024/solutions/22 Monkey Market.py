EXAMPLE_INPUT = '''\
1
2
3
2024
'''

from collections import defaultdict


def generate_secret(secret):
    secret ^= secret * 64
    secret %= 16777216
    secret ^= secret // 32
    secret %= 16777216
    secret ^= secret * 2048
    secret %= 16777216
    return secret


def part_1(input):
    secrets = list(map(int, input.splitlines()))
    result = 0
    for secret in secrets:
        for _ in range(2000):
            secret = generate_secret(secret)
        result += secret
    return result


def part_2(input):
    secrets = list(map(int, input.splitlines()))
    sequences = defaultdict(dict)

    # for each sequence keep track of the first result for that secret
    for idx, secret in enumerate(secrets):
        sequence = []
        prev = int(str(secret)[-1])
        for _ in range(2000):
            secret = generate_secret(secret)
            current = int(str(secret)[-1])

            # update 4 length sequence
            sequence.append(current - prev)
            if len(sequence) > 4:
                sequence.pop(0)
            prev = current

            # add sequence to map
            sequence_map = sequences[tuple(sequence)]
            if idx not in sequence_map:
                sequence_map[idx] = current

    # for each sequence sum all results and return the highest number
    return max([sum(results.values()) for results in sequences.values()])