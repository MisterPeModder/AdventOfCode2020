use itertools::Itertools;
use std::{collections::HashSet, error, fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opcode {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub opcode: Opcode,
    pub param: isize,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Error {
    InvalidInstruction(String),
    InvalidJump,
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidInstruction(insn) => write!(f, "Invalid instruction: {}", insn),
            Error::InvalidJump => f.write_str("Invalid jump"),
        }
    }
}

impl error::Error for Error {}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        #[inline]
        fn parse(s: &str) -> Option<Instruction> {
            let mut parts = s.split_whitespace();
            let name = parts.next()?;
            let param = parts.next()?.parse().ok()?;

            if parts.next().is_some() {
                return None;
            }
            Some(Instruction {
                opcode: match name {
                    "acc" => Some(Opcode::Acc),
                    "jmp" => Some(Opcode::Jmp),
                    "nop" => Some(Opcode::Nop),
                    _ => None,
                }?,
                param,
            })
        }

        parse(s).ok_or_else(|| Error::InvalidInstruction(s.to_owned()))
    }
}

pub struct Computer<'a> {
    acc: isize,
    ip: usize,
    code: &'a [Instruction],
}

impl<'a> Computer<'a> {
    #[inline]
    pub fn new(code: &'a [Instruction]) -> Self {
        Self {
            acc: 0,
            ip: 0,
            code,
        }
    }

    #[inline]
    pub fn acc(&self) -> isize {
        self.acc
    }

    fn run_instruction(&mut self) -> Result<()> {
        let insn = self.code[self.ip];
        let param = insn.param;
        match insn.opcode {
            Opcode::Acc => {
                self.acc += param;
                self.ip += 1;
                Ok(())
            }
            Opcode::Jmp => {
                let new_addr = if param < 0 {
                    self.ip.checked_sub(param.wrapping_abs() as usize)
                } else {
                    self.ip.checked_add(param as usize)
                };
                match new_addr {
                    None => Err(Error::InvalidJump),
                    Some(a) => {
                        self.ip = a;
                        Ok(())
                    }
                }
            }
            Opcode::Nop => {
                self.ip += 1;
                Ok(())
            }
        }
    }

    pub fn run_until_repeat(&mut self) -> Result<bool> {
        let mut ran = HashSet::new();
        let mut current = self.ip;

        self.ip = 0;
        self.acc = 0;
        while !ran.contains(&current) && self.ip < self.code.len() {
            self.run_instruction()?;
            ran.insert(current);
            current = self.ip;
        }
        Ok(self.ip < self.code.len())
    }
}

#[inline(always)]
pub fn parse_code<S: AsRef<str>>(code: S) -> Result<Vec<Instruction>> {
    parse_code_impl(code.as_ref())
}

fn parse_code_impl(code: &str) -> Result<Vec<Instruction>> {
    code.lines().map(|l| Instruction::from_str(l)).try_collect()
}
