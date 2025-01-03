EXAMPLE_INPUT = r'''\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> rx
'''

import itertools
from math import lcm

def parse_input(input):
    modules = {
        'rx': ('', {}, [], [])
    }
    for line in input.splitlines():
        (name, dest) = line.split(' -> ')
        if name == 'broadcaster':
            type = ''
        else:
            type = name[0]
            name = name[1:]
        modules[name] = (type, {}, dest.split(', '), [])

    # set initial state and link inputs
    for module in list(modules.keys()):
        (type, state, dest, _) = modules[module]
        if type == '%': state['v'] = False
        for next in dest:
            next = modules[next]
            if next[0] == '&':
                next[1][module] = False
            next[3].append(module)

    return modules


def update_modules(modules, cb):
    process = [('broadcaster', 'button', False)]
    while process:
        (module, prev, val) = process.pop(0)
        (type, state, dest, _) = modules[module]
        cb(module, prev, val)

        if type == '%':
            if val:
                continue
            state['v'] = not state['v']
            val = state['v']

        elif type == '&':
            state[prev] = val
            val = not all(state.values())

        for next in dest:
            process.append((next, module, val))


def part_1(input):
    modules = parse_input(input)
    result = [0, 0]
    
    for _ in range(1000):
        def f(module, prev, val):
            result[val] += 1
        update_modules(modules, f)
        
    return result[0] * result[1]


def part_2(input):
    modules = parse_input(input)
    target = modules['rx'][3][0]
    targets = modules[target][3]
    results = []

    for i in itertools.count():
        def f(module, prev, val):
            if module == target and prev in targets and val:
                results.append(i + 1)
        update_modules(modules, f)
        if len(targets) == len(results):
            break
        
    return lcm(*results)
