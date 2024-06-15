#![allow(non_snake_case)]
#![allow(dead_code)]

pub struct CPU {
    RegisterA: u8,
    RegisterX: u8,
    RegisterY: u8,
    StackPointer: u8,
    ProgramCounter: u16,
    FlagB: bool,
    FlagC: bool,
    FlagD: bool,
    FlagI: bool,
    FlagN: bool,
    FlagV: bool,
    FlagZ: bool,
}
impl CPU {
    pub fn new() -> Self {
        CPU {
            RegisterA: 0,
            RegisterX: 0,
            RegisterY: 0,
            StackPointer: 0,
            ProgramCounter: 0,
            FlagB: false,
            FlagC: false,
            FlagD: false,
            FlagI: false,
            FlagN: false,
            FlagV: false,
            FlagZ: false,
        }
    }
    pub fn interpret(&mut self, program: Vec<u8>) {
        loop {
            let opcode = program[self.ProgramCounter as usize];
            self.ProgramCounter += 1;

            match opcode {
                _ => todo!()
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
}
