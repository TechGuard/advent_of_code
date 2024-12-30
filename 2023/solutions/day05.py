EXAMPLE_INPUT = '''seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
'''

import re

def part_1(input):
    lines = input.splitlines()
    seeds = [int(x) for x in re.findall(r'\d+', lines[0])]
    maps = []
    for line in lines[1:]:
        if ':' in line:
            maps.append([])
        nums = [int(x) for x in re.findall(r'\d+', line)]
        if len(nums):
            maps[-1].append((nums[0], nums[1], nums[1]+nums[2]))

    result = None
    for x in seeds:
        for map in maps:
            for (dest, mstart, mend) in map:
                if x >= mstart and x <= mend:
                    x = dest + x - mstart
                    break
        if result is None or x < result:
            result = x
    return result


def part_2(input):
    lines = input.splitlines()
    seeds = [int(x) for x in re.findall(r'\d+', lines[0])]
    maps = []
    for line in lines[1:]:
        if ':' in line:
            maps.append([])
        nums = [int(x) for x in re.findall(r'\d+', line)]
        if len(nums):
            maps[-1].append((nums[0], nums[1], nums[1]+nums[2]))

    i = 0
    ranges = []
    while i < len(seeds):
        ranges.append((seeds[i],seeds[i]+seeds[i+1]))
        i += 2

    result = None
    for r in ranges:
        mapped_ranges = [r]
        for map in maps:
            ranges_to_map = mapped_ranges
            mapped_ranges = []
            for (dest, mstart, mend) in map:
                next_to_map = []

                for (rstart, rend) in ranges_to_map:
                    before = (rstart, min(rend, mstart))
                    if before[0] < before[1]:
                        next_to_map.append(before)
                    after = (max(mend, rstart), rend)
                    if after[0] < after[1]:
                        next_to_map.append(after)
                    middle = (max(rstart, mstart), min(rend, mend))
                    if middle[0] < middle[1]:
                        mapped_ranges.append((dest + middle[0] - mstart, dest + middle[1] - mstart))

                ranges_to_map = next_to_map
            mapped_ranges = mapped_ranges + next_to_map

        v = min(mapped_ranges)[0]
        if result is None or v < result:
            result = v
            
    return result