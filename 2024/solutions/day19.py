EXAMPLE_INPUT = '''\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
'''


def find_solutions(designs, patterns):
    cache = {}

    def calc_solutions(design, patterns):
        if design == '':
            return 1
        total = 0
        for pattern in patterns:
            if design.endswith(pattern):
                remainder = design[:-len(pattern)]
                if remainder in cache:
                    result = cache[remainder]
                else:
                    result = calc_solutions(remainder, patterns)
                    cache[remainder] = result
                total += result
        return total

    return [calc_solutions(design, patterns) for design in designs]


def part_1(input):
    patterns, designs = input.split('\n\n')
    patterns = patterns.split(', ')
    designs = designs.splitlines()
    return sum(1 for solutions in find_solutions(designs, patterns) if solutions > 0)


def part_2(input):
    patterns, designs = input.split('\n\n')
    patterns = patterns.split(', ')
    designs = designs.splitlines()
    return sum(find_solutions(designs, patterns))