const C_BYTE_POSITION: u8 = 0;
const Z_BYTE_POSITION: u8 = 1;
const I_BYTE_POSITION: u8 = 2;
const D_BYTE_POSITION: u8 = 3;
const B_BYTE_POSITION: u8 = 4;
const V_BYTE_POSITION: u8 = 6;
const N_BYTE_POSITION: u8 = 7;

impl std::convert::From<Flag> for u8 {
  fn from(flag: Flag) -> u8 {
    (if flag.carry { 1 } else { 0 }) << C_BYTE_POSITION
      | (if flag.zero { 1 } else { 0 }) << Z_BYTE_POSITION
      | (if flag.interrupt { 1 } else { 0 }) << I_BYTE_POSITION
      | (if flag.decimal_mode { 1 } else { 0 }) << D_BYTE_POSITION
      | (if flag.break_command { 1 } else { 0 }) << B_BYTE_POSITION
      | (if flag.overflow { 1 } else { 0 }) << V_BYTE_POSITION
      | (if flag.negative { 1 } else { 0 }) << N_BYTE_POSITION
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
      break_command,
      carry,
      decimal_mode,
      interrupt,
      negative,
      overflow,
      zero,
    }
  }
}
pub struct Flag {
  break_command: bool,
  carry: bool,
  decimal_mode: bool,
  interrupt: bool,
  negative: bool,
  overflow: bool,
  zero: bool,
}
impl Flag {
  pub fn reset(&mut self) {
    self.break_command = false;
    self.carry = false;
    self.decimal_mode = false;
    self.interrupt = false;
    self.negative = false;
    self.overflow = false;
    self.zero = false;
  }
}

pub enum AddressingMode {
  Immediate,
  ZeroPage,
  ZeroPageX,
  ZeroPageY,
  Relative,
  Absolute,
  AbsoluteX,
  AbsoluteY,
  IndexedIndirectX,
  IndirectIndexedY,
}
pub struct Cpu {
  register_a: u8,
  register_x: u8,
  register_y: u8,
  stack_pointer: u8,
  program_counter: u16,
  flag: Flag,
  memory: [u8; 0xffff],
}
impl Cpu {
  pub fn new() -> Self {
    Cpu {
      register_a: 0,
      register_x: 0,
      register_y: 0,
      stack_pointer: 0,
      program_counter: 0,
      flag: Flag::from(0),
      memory: [0; 0xffff],
    }
  }
  pub fn reset(&mut self) {
    self.register_a = 0;
    self.register_x = 0;
    self.register_y = 0;
    self.flag.reset();
    self.program_counter = self.read_mem_16(0xFFFC);
  }
  fn read_mem(&self, address: u16) -> u8 {
    self.memory[address as usize]
  }
  fn write_mem(&mut self, address: u16, data: u8) {
    self.memory[address as usize] = data;
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
  fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
    match mode {
      AddressingMode::Immediate => self.program_counter,
      AddressingMode::ZeroPage => self.read_mem(self.program_counter) as u16,
      AddressingMode::ZeroPageX => self
        .read_mem(self.program_counter)
        .wrapping_add(self.register_x) as u16,
      AddressingMode::ZeroPageY => self
        .read_mem(self.program_counter)
        .wrapping_add(self.register_y) as u16,
      AddressingMode::Absolute => self.read_mem_16(self.program_counter),
      AddressingMode::AbsoluteX => self
        .read_mem_16(self.program_counter)
        .wrapping_add(self.register_x as u16),
      AddressingMode::AbsoluteY => self
        .read_mem_16(self.program_counter)
        .wrapping_add(self.register_y as u16),
      AddressingMode::IndexedIndirectX => {
        self.read_mem_16(self.program_counter.wrapping_add(self.register_x as u16))
      }
      _ => todo!(),
    }
  }
  pub fn load_and_run(&mut self, program: Vec<u8>) {
    self.load(program);
    self.reset();

    self.run();
  }
  fn load(&mut self, program: Vec<u8>) {
    self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
    self.write_mem_16(0xFFFC, 0x8000);
  }
  fn run(&mut self) {
    loop {
      let opcode = self.read_mem(self.program_counter);
      self.program_counter += 1;

      match opcode {
        0xA9 => {
          let param = self.read_mem(self.program_counter);
          self.program_counter += 1;
          self.lda(param);
        }
        0xA2 => {
          let param = self.read_mem(self.program_counter);
          self.program_counter += 1;
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
    self.register_a = value;
    self.update_zero_and_negative_flag(self.register_a);
  }
  fn ldx(&mut self, value: u8) {
    println!("WENT INTO LDX FUNCTION{}", value);
    self.register_x = value;
    self.update_zero_and_negative_flag(self.register_x);
  }
  fn tax(&mut self) {
    self.register_x = self.register_a;
    self.update_zero_and_negative_flag(self.register_x);
  }
  fn update_zero_and_negative_flag(&mut self, result: u8) {
    self.flag.zero = result == 0;
    self.flag.negative = result & 0b1000_0000 != 0
  }

  fn inx(&mut self) {
    self.register_x = self.register_x.wrapping_add(1);
    self.update_zero_and_negative_flag(self.register_x)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_0xa9_lda_immediate_load_data() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0x05);
    assert!(cpu.flag.zero == false);
    assert!(cpu.flag.negative == false);
  }

  #[test]
  fn test_0xa9_lda_zero_flag() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
    assert!(cpu.flag.zero == true);
  }

  #[test]
  fn test_0xa2_ldx_immediate_load_data() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(vec![0xa2, 0x05, 0x00]);
    assert_eq!(cpu.register_x, 0x05);
    assert!(cpu.flag.zero == false);
    assert!(cpu.flag.negative == false);
  }

  #[test]
  fn test_0xa2_ldx_zero_flag() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(vec![0xa2, 0x00, 0x00]);
    assert!(cpu.flag.zero == true);
  }

  #[test]
  fn test_0xa9_lda_negative_flag() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(vec![0xa9, 0xf1, 0x00]);
    assert!(cpu.flag.negative == true);
  }
  #[test]
  fn test_0xaa_tax_move_a_to_x() {
    let mut cpu = Cpu::new();
    cpu.register_a = 10;
    cpu.load_and_run(vec![0xa9, 10, 0xaa, 0x00]);

    assert_eq!(cpu.register_x, 10)
  }
  #[test]
  fn test_0xaa_tax_move_a_to_x_zero_flag() {
    let mut cpu = Cpu::new();
    cpu.register_a = 0;
    cpu.load_and_run(vec![0xa9, 0, 0xaa, 0x00]);

    assert_eq!(cpu.flag.zero, true)
  }
  #[test]
  fn test_0xaa_tax_move_a_to_x_negative_flag() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(vec![0xa9, 0xf1, 0xaa, 0x00]);

    assert_eq!(cpu.flag.negative, true)
  }
  #[test]
  fn test_5_ops_working_together() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 0xc1)
  }

  #[test]
  fn test_inx_overflow() {
    let mut cpu = Cpu::new();
    cpu.register_x = 0xff;
    cpu.load_and_run(vec![0xa2, 0xff, 0xe8, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 1)
  }
}
