use crate::nes::cpu::AddressingMode;

#[allow(dead_code)]
pub struct Opcode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub len: u8,
    pub cycle: u8,
    pub addressing_mode: AddressingMode
}

#[allow(dead_code)]
impl Opcode {
    pub fn new(code: u8, mnemonic: &'static str, len: u8, cycle: u8, addressing_mode: AddressingMode) -> Self {
        Opcode {
            code,
            mnemonic,
            len,
            cycle,
            addressing_mode,
        }
    }
}

//const LDA: Opcode = Opcode::new(0x69, "LDA", 2, 10, AddressingMode::Immediate);