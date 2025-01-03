EXAMPLE_INPUT = '''\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
'''


def solve(springs, sizes):
    cache = {}

    def cached(f, springs, sizes, count):
        key = springs + ''.join([str(x) for x in sizes]) + str(count)
        if key not in cache:
            cache[key] = f(springs, sizes, count)
        return cache[key]
    
    def f(springs, sizes, count):
        for i, c in enumerate(springs):
            if c == '.':
                if not count:
                    continue
                if not sizes or count != sizes[0]:
                    return 0
                sizes = sizes[1:]
                count = 0
            if c == '#':
                count += 1
                if not sizes or count > sizes[0]:
                    return 0
            if c == '?':
                return cached(f, '.' + springs[i + 1:], sizes, count) + cached(f, '#' + springs[i + 1:], sizes, count)
            
        return 0 if sizes else 1
    
    return f(springs + '.', sizes, 0)


def part_1(input):
    result = 0
    for line in input.splitlines():
        springs, sizes = line.split(' ')
        sizes = [int(x) for x in sizes.split(',')]
        result += solve(springs, sizes)
        
    return result


def part_2(input):
    result = 0
    for line in input.splitlines():
        springs, sizes = line.split(' ')
        sizes = [int(x) for x in sizes.split(',')]

        springs = (springs + '?') * 4 + springs
        sizes = sizes * 5

        result += solve(springs, sizes)

    return result