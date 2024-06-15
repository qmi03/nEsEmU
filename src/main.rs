#![allow(non_snake_case)]
#![allow(dead_code)]

const C_BYTE_POSITION: u8 = 0;
const Z_BYTE_POSITION: u8 = 1;
const I_BYTE_POSITION: u8 = 2;
const D_BYTE_POSITION: u8 = 3;
const B_BYTE_POSITION: u8 = 4;
const V_BYTE_POSITION: u8 = 6;
const N_BYTE_POSITION: u8 = 7;

pub struct Flag {
    BreakCommand: bool,
    Carry: bool,
    DecimalMode: bool,
    Interrupt: bool,
    Negative: bool,
    Overflow: bool,
    Zero: bool,
}
pub struct CPU {
    RegisterA: u8,
    RegisterX: u8,
    RegisterY: u8,
    StackPointer: u8,
    ProgramCounter: u16,
    Flag: Flag,
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
                0xA9 => {
                    let param = program[self.ProgramCounter as usize];
                    self.ProgramCounter += 1;
                    self.RegisterA = param;
                }
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
}
