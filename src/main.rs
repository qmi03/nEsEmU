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
    // add code here
}
fn main() {
    println!("Hello, world!");
}
