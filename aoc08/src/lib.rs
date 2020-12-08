pub mod input;

use Instruction::*;

#[derive(Copy, Clone)]
pub enum Instruction {
    ACC(isize),
    JMP(isize),
    NOP(isize),
}

impl Instruction {
    pub fn replace_jmp_nop(&mut self) -> bool {
        match *self {
            JMP(arg) => {
                *self = NOP(arg);
                true
            }
            NOP(arg) => {
                *self = JMP(arg);
                true
            }
            _ => false,
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (instr, arg) = line.split_at(4);
            match instr {
                "acc " => ACC(arg.parse().unwrap()),
                "jmp " => JMP(arg.parse().unwrap()),
                "nop " => NOP(arg.parse().unwrap()),
                _ => panic!("Unknown instruction"),
            }
        })
        .collect()
}

pub enum RunResult {
    InstructionExecutedTwice,
    EndOfInstructions,
}

pub struct GameConsole<'a> {
    instructions: Vec<Option<&'a Instruction>>,
    ip: usize,
    acc: isize,
}

impl<'a> GameConsole<'a> {
    pub fn new(instructions: &'a Vec<Instruction>) -> Self {
        GameConsole {
            instructions: instructions
                .iter()
                .map(|instruction| Some(instruction))
                .collect(),
            ip: 0,
            acc: 0,
        }
    }

    pub fn run(&mut self) -> RunResult {
        while self.ip < self.instructions.len() {
            if let Some(instruction) = self.instructions[self.ip].take() {
                self.ip += 1;
                match instruction {
                    ACC(arg) => self.acc += arg,
                    JMP(arg) => self.ip = (self.ip as isize + arg - 1) as usize,
                    _ => (),
                }
            } else {
                return RunResult::InstructionExecutedTwice;
            }
        }
        RunResult::EndOfInstructions
    }

    pub fn acc(&self) -> isize {
        self.acc
    }
}
