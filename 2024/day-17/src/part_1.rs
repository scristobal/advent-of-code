use anyhow::Result;

#[derive(Debug)]
struct Machine {
    register_a: i32,
    register_b: i32,
    register_c: i32,
    program: Vec<i32>,
    instruction_pointer: i32,
    out_buffer: Vec<i32>,
}

impl Machine {
    fn run(&mut self) {
        while self.execute() {}
    }

    fn execute(&mut self) -> bool {
        if self.instruction_pointer >= self.program.len().try_into().unwrap() {
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

    fn combo_op(&self, operand: i32) -> i32 {
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
}

fn parse(input: &'static str) -> Machine {
    let (registers, program) = input.split_once("\n\n").unwrap();

    let mut registers = registers.lines();

    let register_a: i32 = registers
        .next()
        .unwrap()
        .strip_prefix("Register A: ")
        .map(|s| s.parse().unwrap())
        .unwrap();

    let register_b: i32 = registers
        .next()
        .unwrap()
        .strip_prefix("Register B: ")
        .map(|s| s.parse().unwrap())
        .unwrap();

    let register_c: i32 = registers
        .next()
        .unwrap()
        .strip_prefix("Register C: ")
        .map(|s| s.parse().unwrap())
        .unwrap();

    let program: Vec<i32> = program
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

    machine.run();

    Ok(machine.read_out())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_sample1() {
        #[rustfmt::skip]
        let result = solve(
"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
").unwrap();

        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn solve_sample2() {
        #[rustfmt::skip]
        let result = solve(
"Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4
").unwrap();

        assert_eq!(result, "0,1,2");
    }

    #[test]
    fn solve_sample3() {
        #[rustfmt::skip]
        let result = solve(
"Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
").unwrap();

        assert_eq!(result, "4,2,5,6,7,7,7,7,3,1,0");
    }

    #[test]
    fn solve_sample4() {
        #[rustfmt::skip]
       let mut m  = parse(
"Register A: 0
Register B: 0
Register C: 9

Program: 2,6
");

        m.run();

        assert_eq!(m.register_b, 1);
    }

    #[test]
    fn solve_sample5() {
        #[rustfmt::skip]
       let mut m  = parse(
"Register A: 0
Register B: 29
Register C: 0

Program: 1,7
");

        m.run();

        assert_eq!(m.register_b, 26);
    }

    #[test]
    fn solve_sample6() {
        #[rustfmt::skip]
       let mut m  = parse(
"Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0
");

        m.run();

        assert_eq!(m.register_b, 44354);
    }
}
