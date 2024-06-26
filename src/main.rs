#![allow(non_snake_case)]
#![allow(dead_code)]

use std::u8;

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
impl Flag {
    pub fn reset(&mut self) {
        self.BreakCommand = false;
        self.Carry = false;
        self.DecimalMode = false;
        self.Interrupt = false;
        self.Negative = false;
        self.Overflow = false;
        self.Zero = false;
    }
}
pub struct CPU {
    RegisterA: u8,
    RegisterX: u8,
    RegisterY: u8,
    StackPointer: u8,
    ProgramCounter: u16,
    Flag: Flag,
    Memory: [u8; 0xffff],
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
            Memory: [0; 0xffff],
        }
    }
    pub fn reset(&mut self) {
        self.RegisterA = 0;
        self.RegisterX = 0;
        self.RegisterY = 0;
        self.Flag.reset();
        self.ProgramCounter = self.read_mem_16(0xFFFC);
    }
    fn read_mem(&self, address: u16) -> u8 {
        self.Memory[address as usize]
    }
    fn write_mem(&mut self, address: u16, data: u8) {
        self.Memory[address as usize] = data;
    }
    fn read_mem_16(&self, address: u16) -> u16 {
        let least_significant_byte = self.read_mem(address) as u16;
        let most_significant_byte = self.read_mem(address + 1) as u16;
        ((most_significant_byte << 8) | least_significant_byte) as u16
    }
    fn write_mem_16(&mut self, address: u16, data: u16) {
        let least_significant_byte = (data & 0x00ff) as u8;
        let most_significant_byte = (data >> 8) as u8;
        self.write_mem(address, least_significant_byte);
        self.write_mem(address + 1, most_significant_byte);
    }
    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();

        self.run();
    }
    fn load(&mut self, program: Vec<u8>) {
        self.Memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.write_mem_16(0xFFFC, 0x8000);
    }
    fn run(&mut self) {
        loop {
            let opcode = self.read_mem(self.ProgramCounter);
            self.ProgramCounter += 1;

            match opcode {
                0xA9 => {
                    let param = self.read_mem(self.ProgramCounter);
                    self.ProgramCounter += 1;
                    self.lda(param);
                }
                0xA2 => {
                    let param = self.read_mem(self.ProgramCounter);
                    self.ProgramCounter += 1;
                    self.ldx(param);
                }
                0xAA => self.tax(),
                0xE8 => self.inx(),
                0x00 => return,
                _ => todo!(),
            }
        }
    }
    fn lda(&mut self, value: u8) {
        self.RegisterA = value;
        self.update_zero_and_negative_flag(self.RegisterA);
    }
    fn ldx(&mut self, value: u8) {
        println!("WENT INTO LDX FUNCTION{}", value);
        self.RegisterX = value;
        self.update_zero_and_negative_flag(self.RegisterX);
    }
    fn tax(&mut self) {
        self.RegisterX = self.RegisterA;
        self.update_zero_and_negative_flag(self.RegisterX);
    }
    fn update_zero_and_negative_flag(&mut self, result: u8) {
        self.Flag.Zero = result == 0;
        self.Flag.Negative = result & 0b1000_0000 != 0
    }

    fn inx(&mut self) {
        self.RegisterX = self.RegisterX.wrapping_add(1);
        self.update_zero_and_negative_flag(self.RegisterX)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.RegisterA, 0x05);
        assert!(cpu.Flag.Zero == false);
        assert!(cpu.Flag.Negative == false);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.Flag.Zero == true);
    }

    #[test]
    fn test_0xa2_ldx_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x05, 0x00]);
        assert_eq!(cpu.RegisterX, 0x05);
        assert!(cpu.Flag.Zero == false);
        assert!(cpu.Flag.Negative == false);
    }

    #[test]
    fn test_0xa2_ldx_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x00, 0x00]);
        assert!(cpu.Flag.Zero == true);
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xf1, 0x00]);
        assert!(cpu.Flag.Negative == true);
    }
    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.RegisterA = 10;
        cpu.load_and_run(vec![0xa9, 10, 0xaa, 0x00]);

        assert_eq!(cpu.RegisterX, 10)
    }
    #[test]
    fn test_0xaa_tax_move_a_to_x_zero_flag() {
        let mut cpu = CPU::new();
        cpu.RegisterA = 0;
        cpu.load_and_run(vec![0xa9, 0, 0xaa, 0x00]);

        assert_eq!(cpu.Flag.Zero, true)
    }
    #[test]
    fn test_0xaa_tax_move_a_to_x_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xf1, 0xaa, 0x00]);

        assert_eq!(cpu.Flag.Negative, true)
    }
    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.RegisterX, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.RegisterX = 0xff;
        cpu.load_and_run(vec![0xa2, 0xff, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.RegisterX, 1)
    }
}
fn main() {
    println!("Hello, world!");
}
