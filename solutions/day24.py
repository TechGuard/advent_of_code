EXAMPLE_INPUT = '''\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
'''


def part_1(input):
    states_input, gates = input.split('\n\n')
    gates = [line.split() for line in gates.splitlines()]
    states = {}
    num_z = 0
    for line in states_input.splitlines():
        key, value = line.split(': ')
        states[key] = value == '1'

    while gates:
        for gate in gates:
            if gate[0] in states and gate[2] in states:
                gates.remove(gate)
                break

        lhs, op, rhs, _, out = gate
        if op == 'AND':
            states[out] = states[lhs] and states[rhs]
        elif op == 'OR':
            states[out] = states[lhs] or states[rhs]
        elif op == 'XOR':
            states[out] = states[lhs] != states[rhs]
        else:
            assert False

        if out[0] == 'z' and states[out]:
            num_z |= 1 << int(out[1:])
            
    return num_z


def part_2(input):
    states_input, gates = input.split('\n\n')
    gates = [line.split() for line in gates.splitlines()]
    length = max([int(line.split(': ')[0][1:]) for line in states_input.splitlines()])

    def find(f_lhs, f_op, f_rhs):
        for i in range(len(gates)):
            lhs, op, rhs, _, out = gates[i]
            if f_op != op:
                continue
            if (f_lhs == lhs and f_rhs == rhs) or (f_lhs == rhs and f_rhs == lhs):
                return out, i
        return None, None

    def find_index_out(f_out):
        for i in range(len(gates)):
            if gates[i][4] == f_out:
                return i

    results = []

    # initial half adder
    assert find('y00', 'XOR', 'x00')[0] == 'z00'
    carry, _ = find('y00', 'AND', 'x00')

    index = 1
    while index < length + 1:
        # full adder is two half adders
        x = 'x{:02d}'.format(index)
        y = 'y{:02d}'.format(index)
        z = 'z{:02d}'.format(index)

        # first half adder
        and00, and00_i = find(y, 'AND', x)
        xor00, xor00_i = find(y, 'XOR', x)

        # second half adder
        and01, _ = find(xor00, 'AND', carry)
        if and01 is None:
            gates[xor00_i][4], gates[and00_i][4] = gates[and00_i][4], gates[xor00_i][4]
            results.append(and00)
            results.append(xor00)
            continue

        xor01, xor01_i = find(xor00, 'XOR', carry)
        if xor01 != z:
            z_i = find_index_out(z)
            gates[xor01_i][4], gates[z_i][4] = gates[z_i][4], gates[xor01_i][4]
            results.append(z)
            results.append(xor01)
            continue
            
        # carry output for next index
        carry, _ = find(and00, 'OR', and01)
        index += 1

    return ','.join(sorted(results))