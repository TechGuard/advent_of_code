EXAMPLE_INPUT = '''2333133121414131402
'''


def solve(input, next):
    diskmap = []
    free = False
    for c in input.strip():
        diskmap.append({'id': '.' if free else str(len(diskmap) // 2), 'count': int(c), 'free': free})
        free = not free

    target = 0
    source = len(diskmap) - 1

    while True:
        next_res = next(diskmap, source, target)
        if next_res is None:
            break

        source, target = next_res

        # swap
        src = diskmap[source]
        tgt = diskmap[target]

        if src['count'] <= tgt['count']:
            remaining = tgt['count'] - src['count']
            tgt['id'] = src['id']
            tgt['count'] = src['count']
            tgt['free'] = False
            src['id'] = '.'
            src['free'] = True
            if remaining:
                diskmap.insert(target + 1, {'id': '.', 'count': remaining, 'free': True})
                source += 1
        else:
            remaining = src['count'] - tgt['count']
            tgt['id'] = src['id']
            src['count'] = remaining
            tgt['free'] = False

    pos = 0
    result = 0
    for b in diskmap:
        if not b['free']:
            factor = b['count'] * (pos + ((b['count'] - 1) / 2))
            result += int(b['id']) * factor
        pos += b['count']
        
    return int(result)


def part_1(input):
    def next(diskmap, source, target):
        # find target
        for i in range(target, len(diskmap)):
            if diskmap[i]['free']:
                target = i
                break
        if not diskmap[target]['free']:
            return None

        # find source
        for i in range(source, -1, -1):
            if not diskmap[i]['free']:
                source = i
                break
        if source < target:
            return None

        return source, target
        
    return solve(input, next)


def part_2(input):
    def next(diskmap, source, target):
        while True:
            # find source
            for i in range(source, -1, -1):
                if not diskmap[i]['free']:
                    source = i
                    break
            if diskmap[source]['free']:
                return None

            # find target
            target = 0
            for i in range(target, len(diskmap)):
                if diskmap[i]['free'] and diskmap[source]['count'] <= diskmap[i]['count']:
                    target = i
                    break
            if not diskmap[target]['free'] or source < target:
                source -= 1
                continue
            
            return source, target
        
    return solve(input, next)