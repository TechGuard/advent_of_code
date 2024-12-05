EXAMPLE_INPUT = '''47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
'''


def parse_input(input):
    rules, updates = input.split('\n\n')
    return ([rule.split('|') for rule in rules.splitlines()], [update.split(',') for update in updates.splitlines()])


def is_valid_update(rules, update):
    for before, after in rules:
        if before in update and after in update:
            if update.index(before) > update.index(after):
                return False
    return True


def part_1(input):
    rules, updates = parse_input(input)
    result = 0
    for update in updates:
        if is_valid_update(rules, update):
            result += int(update[int(len(update)/2)])
    return result


def part_2(input):
    rules, updates = parse_input(input)
    result = 0
    for update in updates:
        if is_valid_update(rules, update):
            continue

        new_update = []
        for n in update:
            i = 0
            for before, after in rules:
                if n == before:
                    if after in new_update:
                        i = min(i, new_update.index(after))
                if n == after:
                    if before in new_update:
                        i = max(i, new_update.index(before)+1)
            new_update.insert(i, n)

        result += int(new_update[int(len(new_update)/2)])
    return result