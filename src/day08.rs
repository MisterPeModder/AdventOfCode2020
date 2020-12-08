use crate::util::computer::{self, Computer, Opcode};

#[aoc(day08, part1)]
pub fn day08_part1(input: &str) -> isize {
    let code = computer::parse_code(input).unwrap();
    let mut computer = Computer::new(&code);

    computer.run_until_repeat().unwrap();
    computer.acc()
}

#[aoc(day08, part2)]
pub fn day08_part2(input: &str) -> isize {
    let mut code = computer::parse_code(input).unwrap();

    for i in 0..code.len() {
        let opcode = code[i].opcode;
        let other = match opcode {
            Opcode::Jmp => Opcode::Nop,
            Opcode::Nop => Opcode::Jmp,
            _ => continue,
        };
        let mut computer;

        code[i].opcode = other;
        computer = Computer::new(&code);
        if !computer.run_until_repeat().unwrap() {
            return computer.acc();
        }
        code[i].opcode = opcode;
    }
    panic!("cannot find non-looping solution")
}
