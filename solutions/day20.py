# EXAMPLE_INPUT = r'''broadcaster -> a, b, c
# %a -> b
# %b -> c
# %c -> inv
# &inv -> a
# '''
EXAMPLE_INPUT = r'''broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> rx
'''


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


def part_1(input):
    modules = parse_input(input)
    result = [0, 0]

    for _ in range(1000):
        process = [('broadcaster', 'button', False)]
        while process:
            (module, prev, val) = process.pop(0)
            (type, state, dest, _) = modules[module]
            result[val] += 1

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

    return result[0] * result[1]


def part_2(input):
    modules = parse_input(input)
    return
    
    # def f(seen, module):
    #     (type, state, dest, input) = modules[module]
    #     if module in seen:
    #         print('loop', seen, module)
    #         return
    #     if not input:
    #         print(seen, module)
    #     for prev in input:
    #         f(seen + [module], prev)
    # f([], 'rx')

    '''
    broadcaster -> a
    %a -> inv, con
    &inv -> b
    %b -> con
    &con -> output

    output
      con
        b
          inv
            a
              broadcaster
        a
          broadcaster
    '''