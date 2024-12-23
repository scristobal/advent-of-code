use anyhow::Result;

#[derive(Debug)]
struct Machine {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    program: Vec<i64>,
    instruction_pointer: i64,
    out_buffer: Vec<i64>,
}

impl Machine {
    fn run(&mut self) {
        while self.execute() {}
    }

    fn execute(&mut self) -> bool {
        if (self.instruction_pointer + 1) >= (self.program.len().try_into().unwrap()) {
            return false;
        }

        let pointer: usize = self.instruction_pointer.try_into().unwrap();

        let opcode = self.program[pointer];
        let operand = self.program[pointer + 1];

        match (opcode, operand) {
            (0, operand) => self.register_a >>= self.combo_op(operand),
            (1, operand) => self.register_b ^= operand,
            (2, operand) => self.register_b = self.combo_op(operand).rem_euclid(8),
            (3, operand) if self.register_a != 0 => self.instruction_pointer = operand,
            (3, _) if self.register_a == 0 => (),
            (4, _) => self.register_b ^= self.register_c,
            (5, operand) => self.out_buffer.push(self.combo_op(operand).rem_euclid(8)),
            (6, operand) => self.register_b = self.register_a >> self.combo_op(operand),
            (7, operand) => self.register_c = self.register_a >> self.combo_op(operand),
            _ => unreachable!(),
        };

        if opcode != 3 || self.register_a == 0 {
            self.instruction_pointer += 2;
        };

        true
    }

    fn combo_op(&self, operand: i64) -> i64 {
        match operand {
            x if x < 0 => unreachable!(),
            x if x <= 3 => x,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => unreachable!(),
        }
    }

    fn read_out(&self) -> String {
        let res: Vec<_> = self.out_buffer.iter().map(|v| v.to_string()).collect();
        res.join(",")
    }

    fn read_program(&self) -> String {
        let res: Vec<_> = self.program.iter().map(|v| v.to_string()).collect();
        res.join(",")
    }

    fn restart(&mut self) {
        self.register_a = 0;
        self.register_b = 0;
        self.register_c = 0;
        self.instruction_pointer = 0;
        self.out_buffer.clear();
    }
}

fn parse(input: &'static str) -> Machine {
    let (registers, program) = input.split_once("\n\n").unwrap();

    let mut registers = registers.lines();

    let register_a: i64 = registers
        .next()
        .unwrap()
        .strip_prefix("Register A: ")
        .map(|s| s.parse().unwrap())
        .unwrap();

    let register_b: i64 = registers
        .next()
        .unwrap()
        .strip_prefix("Register B: ")
        .map(|s| s.parse().unwrap())
        .unwrap();

    let register_c: i64 = registers
        .next()
        .unwrap()
        .strip_prefix("Register C: ")
        .map(|s| s.parse().unwrap())
        .unwrap();

    let program: Vec<i64> = program
        .strip_prefix("Program: ")
        .unwrap()
        .replace("\n", "")
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    Machine {
        register_a,
        register_b,
        register_c,
        program,
        instruction_pointer: 0,
        out_buffer: Vec::new(),
    }
}

pub fn solve(input: &'static str) -> Result<String> {
    let mut machine = parse(input);

    let full_program = machine.program.clone();
    let program_len = full_program.len();

    // remove the loop from the program
    machine.program.pop();
    machine.program.pop();

    let mut register_a = 0;

    for program_ind in (0..program_len).rev() {
        register_a <<= 3;

        for a_bits in 0..=8 {
            machine.restart();
            machine.register_a = register_a + a_bits;

            machine.run();

            if *machine.out_buffer.first().unwrap() == full_program[program_ind] {
                register_a += a_bits;
                break;
            }
        }

        assert_eq!(machine.out_buffer.first(), Some(&full_program[program_ind]))
    }

    machine.restart();
    machine.register_a = register_a;

    // put back the loop into the program
    machine.program.push(3);
    machine.program.push(0);

    machine.run();

    assert_eq!(machine.read_out(), machine.read_program());

    Ok(register_a.to_string())
}
