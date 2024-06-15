#![allow(non_snake_case)]
#![allow(dead_code)]

const C_BYTE_POSITION: u8 = 0;
const Z_BYTE_POSITION: u8 = 1;
const I_BYTE_POSITION: u8 = 2;
const D_BYTE_POSITION: u8 = 3;
const B_BYTE_POSITION: u8 = 4;
const V_BYTE_POSITION: u8 = 6;
const N_BYTE_POSITION: u8 = 7;

impl std::convert::From<Flag> for u8 {
    fn from(flag: Flag) -> u8 {
        (if flag.Carry { 1 } else { 0 }) << C_BYTE_POSITION
            | (if flag.Zero { 1 } else { 0 }) << Z_BYTE_POSITION
            | (if flag.Interrupt { 1 } else { 0 }) << I_BYTE_POSITION
            | (if flag.DecimalMode { 1 } else { 0 }) << D_BYTE_POSITION
            | (if flag.BreakCommand { 1 } else { 0 }) << B_BYTE_POSITION
            | (if flag.Overflow { 1 } else { 0 }) << V_BYTE_POSITION
            | (if flag.Negative { 1 } else { 0 }) << N_BYTE_POSITION
    }
}

impl std::convert::From<u8> for Flag {
    fn from(value: u8) -> Self {
        let carry = ((value >> C_BYTE_POSITION) & 0b1) != 0;
        let zero = ((value >> Z_BYTE_POSITION) & 0b1) != 0;
        let interrupt = ((value >> I_BYTE_POSITION) & 0b1) != 0;
        let decimal_mode = ((value >> D_BYTE_POSITION) & 0b1) != 0;
        let break_command = ((value >> B_BYTE_POSITION) & 0b1) != 0;
        let overflow = ((value >> V_BYTE_POSITION) & 0b1) != 0;
        let negative = ((value >> N_BYTE_POSITION) & 0b1) != 0;

        Flag {
            BreakCommand: break_command,
            Carry: carry,
            DecimalMode: decimal_mode,
            Interrupt: interrupt,
            Negative: negative,
            Overflow: overflow,
            Zero: zero,
        }
    }
}
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
            Flag: Flag::from(0),
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

                    self.Flag.Zero = self.RegisterA == 0;
                    self.Flag.Negative = self.RegisterA & 0b1000_0000 != 0
                }
                _ => todo!(),
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
}
