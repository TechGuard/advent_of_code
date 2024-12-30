EXAMPLE_INPUT = '''\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
'''

A, B, C = range(3)


class Program:
    def execute(self, registers, data):
        instructions = [ self.adv, self.bxl, self.bst, self.jnz, self.bxc, self.out, self.bdv, self.cdv ]
        self.registers = registers
        self.out_data = []
        self.ptr = 0
        while self.ptr < len(data):
            opcode, operand = data[self.ptr:self.ptr+2]
            assert 0 <= opcode < len(instructions)
            self.ptr += 2
            instructions[opcode](operand)
        return self.out_data

    def adv(self, operand):
        self.registers[A] = self.registers[A] // pow(2, self.combo_operand(operand))

    def bxl(self, operand):
        self.registers[B] = self.registers[B] ^ operand

    def bst(self, operand):
        self.registers[B] = self.combo_operand(operand) % 8

    def jnz(self, operand):
        if self.registers[A] != 0:
            self.ptr = operand

    def bxc(self, operand):
        self.registers[B] = self.registers[B] ^ self.registers[C]

    def out(self, operand):
        self.out_data.append(self.combo_operand(operand) % 8)

    def bdv(self, operand):
        self.registers[B] = self.registers[A] // pow(2, self.combo_operand(operand))

    def cdv(self, operand):
        self.registers[C] = self.registers[A] // pow(2, self.combo_operand(operand))

    def combo_operand(self, operand):
        if 0 <= operand <= 3:
            return operand
        elif 4 <= operand <= 6:
            return self.registers[operand - 4]
        raise 'Invalid combo operand'


def part_1(input):
    input_registers, input_program = input.split('\n\n')
    registers = list(map(int, [line.split(': ')[1] for line in input_registers.splitlines()]))
    program_data = list(map(int, input_program[9:].split(',')))
    program = Program()
    
    # test examples if running with example input
    if input == EXAMPLE_INPUT:
        assert program.execute([0,0,9], [2,6]) == []
        assert program.registers[B] == 1
        assert program.execute([10,0,0], [5,0,5,1,5,4]) == [0,1,2]
        assert program.execute([2024,0,0], [0,1,5,4,3,0]) == [4,2,5,6,7,7,7,7,3,1,0]
        assert program.registers[A] == 0
        assert program.execute([0,29,0], [1,7]) == []
        assert program.registers[B] == 26
        assert program.execute([0,2024,43690], [4,0]) == []
        assert program.registers[B] == 44354

    out_data = program.execute(registers, program_data)
    return ','.join(map(str, out_data))


def part_2(input):
    program_data = list(map(int, input.split('\n\n')[1][9:].split(',')))

    def find_source_value(offset, prev_value):
        # there are no more values to test
        if offset == 0:
            return prev_value
        
        # take the prev_value and test all 3 bit values
        for i in range(8):
            test_value = prev_value * 8 + i
            out_data = Program().execute([test_value,0,0], program_data)
            # If the beginning out the output matches the last value in program_data we have a match
            # we can then use that as our starting value to find the next value in program_data
            # work our way backwards to the beginning
            if out_data[0] == program_data[offset - 1]:
                # see if this branch leads to a valid result
                result = find_source_value(offset - 1, test_value)
                if result is not None:
                    return result
    
    return find_source_value(len(program_data), 0)