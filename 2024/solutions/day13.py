EXAMPLE_INPUT = '''Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
'''


def solve_machine(machine, offset):
    move_a, move_b, goal = machine
    goal = (goal[0] + offset, goal[1] + offset)

    # solve for a and b
    # a * move_a[0] + b * move_b[0] = goal[0]
    # a * move_a[1] + b * move_b[1] = goal[1]
    a = int((goal[0] * move_b[1] - goal[1] * move_b[0]) / (move_a[0] * move_b[1] - move_a[1] * move_b[0]))
    b = int((move_a[0] * goal[1] - move_a[1] * goal[0]) / (move_a[0] * move_b[1] - move_a[1] * move_b[0]))
    
    result = (move_a[0] * a + move_b[0] * b, move_a[1] * a + move_b[1] * b)
    if result == goal:
        return a * 3 + b
    else:
        return 0


def solve(input, part):
    machines = [
        tuple(
            tuple([int(val[2:]) for val in line.split(': ')[1].split(', ')])
            for line in lines.splitlines()
        )
        for lines in input.split('\n\n')
    ]
    return sum(solve_machine(machine, 10000000000000 if part == 2 else 0) for machine in machines)
    

def part_1(input):
    return solve(input, 1)


def part_2(input):
    return solve(input, 2)