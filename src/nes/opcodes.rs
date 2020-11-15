use crate::nes::cpu::AddressingMode;

#[allow(dead_code)]
pub struct Opcode {
    pub code: u8,
    pub len: u8,
    pub cycle: u8,
    pub addressing_mode: AddressingMode
}

#[allow(dead_code)]
impl Opcode {
    pub const  fn new(code: u8, len: u8, cycle: u8, addressing_mode: AddressingMode) -> Self {
        Opcode {
            code,
            len,
            cycle,
            addressing_mode,
        }
    }

    pub const fn match_opcode(code: u8) -> Self {
        match code {
            //ADC
            0x69 => Opcode::new(code, 2, 2, AddressingMode::Immediate),
            0x65 => Opcode::new(code, 2, 3, AddressingMode::ZeroPage),
            0x75 => Opcode::new(code, 2, 4, AddressingMode::ZeroPageX),
            0x6D => Opcode::new(code, 3, 4, AddressingMode::Absolute),
            0x7D => Opcode::new(code, 3, 4, AddressingMode::AbsoluteX),
            0x79 => Opcode::new(code, 3, 4, AddressingMode::AbsoluteY),
            0x61 => Opcode::new(code, 2, 6, AddressingMode::IndirectX),
            0x71 => Opcode::new(code, 2, 5, AddressingMode::IndirectY),

            //AND
            0x29 => Opcode::new(code, 2, 2, AddressingMode::Immediate),
            0x25 => Opcode::new(code, 2, 3, AddressingMode::ZeroPage),
            0x35 => Opcode::new(code, 2, 4, AddressingMode::ZeroPageX),
            0x2D => Opcode::new(code, 3, 4, AddressingMode::Absolute),
            0x3D => Opcode::new(code, 3, 4, AddressingMode::AbsoluteX),
            0x39 => Opcode::new(code, 3, 4, AddressingMode::AbsoluteY),
            0x21 => Opcode::new(code, 2, 6, AddressingMode::IndirectX),
            0x31 => Opcode::new(code, 2, 5, AddressingMode::IndirectY),

            //ASL
            0x0A => Opcode::new(code, 1, 2, AddressingMode::Accumulator),
            0x06 => Opcode::new(code, 2, 5, AddressingMode::ZeroPage),
            0x16 => Opcode::new(code, 2, 6, AddressingMode::ZeroPageX),
            0x0E => Opcode::new(code, 3, 6, AddressingMode::Absolute),
            0x1E => Opcode::new(code, 3, 7, AddressingMode::AbsoluteX),

            //BCC
            0x90 => Opcode::new(code, 2, 2, AddressingMode::Relative),
            //BEQ
            0xF0 => Opcode::new(code, 2, 2, AddressingMode::Relative),

            //BIT
            0x24 => Opcode::new(code, 2, 3, AddressingMode::ZeroPage),
            0x2C => Opcode::new(code, 3, 4, AddressingMode::Absolute),

            //BMI
            0x30 => Opcode::new(code, 2, 2, AddressingMode::Relative),
            //BNE
            0xD0 => Opcode::new(code, 2, 2, AddressingMode::Relative),
            //BPL
            0x10 => Opcode::new(code, 2, 2, AddressingMode::Relative),
            //BRK
            0x00 => Opcode::new(code, 1, 7, AddressingMode::Implied),
            //BVC
            0x50 => Opcode::new(code, 2, 2, AddressingMode::Relative),
            //BVS
            0x70 => Opcode::new(code, 2, 2, AddressingMode::Relative),

            //CLC
            0x18 => Opcode::new(code, 1, 2, AddressingMode::Implied),
            //CLD
            0xD8 => Opcode::new(code, 1, 2, AddressingMode::Implied),
            //CLI
            0x58 => Opcode::new(code, 1, 2, AddressingMode::Implied),
            //CLV
            0xB8 => Opcode::new(code, 1, 2, AddressingMode::Implied),

            //CMP
            0xC9 => Opcode::new(code, 2, 2, AddressingMode::Immediate),
            0xC5 => Opcode::new(code, 2, 3, AddressingMode::ZeroPage),
            0xD5 => Opcode::new(code, 2, 4, AddressingMode::ZeroPageX),
            0xCD => Opcode::new(code, 3, 4, AddressingMode::Absolute),
            0xDD => Opcode::new(code, 3, 4, AddressingMode::AbsoluteX),
            0xD9 => Opcode::new(code, 3, 4, AddressingMode::AbsoluteY),
            0xC1 => Opcode::new(code, 2, 6, AddressingMode::IndirectX),
            0xD1 => Opcode::new(code, 2, 5, AddressingMode::IndirectY),

            //CPX
            0xE0 => Opcode::new(code, 2, 2, AddressingMode::Immediate),
            0xE4 => Opcode::new(code, 2, 3, AddressingMode::ZeroPage),
            0xEC => Opcode::new(code, 3, 4, AddressingMode::Absolute),

            //CPY
            0xC0 => Opcode::new(code, 2, 2, AddressingMode::Immediate),
            0xC4 => Opcode::new(code, 2, 3, AddressingMode::ZeroPage),
            0xCC => Opcode::new(code, 3, 4, AddressingMode::Absolute),

            //DEC
            0xC6 => Opcode::new(code, 2, 5, AddressingMode::ZeroPage),
            0xD6 => Opcode::new(code, 2, 6, AddressingMode::ZeroPageX),
            0xCE => Opcode::new(code, 3, 6, AddressingMode::Absolute),
            0xDE => Opcode::new(code, 3, 7, AddressingMode::AbsoluteX),

            //DEX
            0xCA => Opcode::new(code, 1, 2, AddressingMode::Implied),
            //DEY
            0x88 => Opcode::new(code, 1, 2, AddressingMode::Implied),

            //EOR
            0x49 => Opcode::new(code, 2, 2, AddressingMode::Immediate),
            0x45 => Opcode::new(code, 2, 3, AddressingMode::ZeroPage),
            0x55 => Opcode::new(code, 2, 4, AddressingMode::ZeroPageX),
            0x4D => Opcode::new(code, 3, 4, AddressingMode::Absolute),
            0x5D => Opcode::new(code, 3, 4, AddressingMode::AbsoluteX),
            0x59 => Opcode::new(code, 3, 4, AddressingMode::AbsoluteY),
            0x41 => Opcode::new(code, 2, 6, AddressingMode::IndirectX),
            0x51 => Opcode::new(code, 2, 5, AddressingMode::IndirectY),

            //INC
            0xE6 => Opcode::new(code, 2, 5, AddressingMode::ZeroPage),
            0xF6 => Opcode::new(code, 2, 6, AddressingMode::ZeroPageX),
            0xEE => Opcode::new(code, 3, 6, AddressingMode::Absolute),
            0xFE => Opcode::new(code, 3, 7, AddressingMode::AbsoluteX),

            //INX
            0xE8 => Opcode::new(code, 1, 2, AddressingMode::Implied),
            //INY
            0xC8 => Opcode::new(code, 1, 2, AddressingMode::Implied),

            //JMP
            0x4C => Opcode::new(code, 3, 3, AddressingMode::Absolute),
            0x6C => Opcode::new(code, 3, 5, AddressingMode::Indirect),

            //JSR
            0x20 => Opcode::new(code, 3, 5, AddressingMode::Absolute),

            //LDA
            0xA9 => Opcode::new(code, 2, 2, AddressingMode::Immediate),
            0xA5 => Opcode::new(code, 2, 3, AddressingMode::ZeroPage),
            0xB5 => Opcode::new(code, 2, 4, AddressingMode::ZeroPageX),
            0xAD => Opcode::new(code, 3, 4, AddressingMode::Absolute),
            0xBD => Opcode::new(code, 3, 4, AddressingMode::AbsoluteX),
            0xB9 => Opcode::new(code, 3, 4, AddressingMode::AbsoluteY),
            0xA1 => Opcode::new(code, 2, 6, AddressingMode::IndirectX),
            0xB1 => Opcode::new(code, 2, 5, AddressingMode::IndirectY),

            //LDX
            0xA2 => Opcode::new(code, 2, 2, AddressingMode::Immediate),
            0xA6 => Opcode::new(code, 2, 3, AddressingMode::ZeroPage),
            0xB6 => Opcode::new(code, 2, 4, AddressingMode::ZeroPageY),
            0xAE => Opcode::new(code, 3, 4, AddressingMode::Absolute),
            0xBE => Opcode::new(code, 3, 4, AddressingMode::AbsoluteY),

            //LDY
            0xA0 => Opcode::new(code, 2, 2, AddressingMode::Immediate),
            0xA4 => Opcode::new(code, 2, 3, AddressingMode::ZeroPage),
            0xB4 => Opcode::new(code, 2, 4, AddressingMode::ZeroPageX),
            0xAC => Opcode::new(code, 3, 4, AddressingMode::Absolute),
            0xBC => Opcode::new(code, 3, 4, AddressingMode::AbsoluteX),

            //LSR
            0x4A => Opcode::new(code, 1, 2, AddressingMode::Accumulator),
            0x46 => Opcode::new(code, 2, 5, AddressingMode::ZeroPage),
            0x56 => Opcode::new(code, 2, 6, AddressingMode::ZeroPageX),
            0x4E => Opcode::new(code, 3, 6, AddressingMode::Absolute),
            0x5E => Opcode::new(code, 3, 7, AddressingMode::AbsoluteX),

            //NOP
            0xEA => Opcode::new(code, 1, 2, AddressingMode::Implied),

            //EOR
            0x09 => Opcode::new(code, 2, 2, AddressingMode::Immediate),
            0x05 => Opcode::new(code, 2, 3, AddressingMode::ZeroPage),
            0x15 => Opcode::new(code, 2, 4, AddressingMode::ZeroPageX),
            0x0D => Opcode::new(code, 3, 4, AddressingMode::Absolute),
            0x1D => Opcode::new(code, 3, 4, AddressingMode::AbsoluteX),
            0x19 => Opcode::new(code, 3, 4, AddressingMode::AbsoluteY),
            0x01 => Opcode::new(code, 2, 6, AddressingMode::IndirectX),
            0x11 => Opcode::new(code, 2, 5, AddressingMode::IndirectY),

            //PHA
            0x48 => Opcode::new(code, 1, 3, AddressingMode::Implied),
            //PHP
            0x08 => Opcode::new(code, 1, 3, AddressingMode::Implied),
            //PLA
            0x68 => Opcode::new(code, 1, 4, AddressingMode::Implied),
            //PLP
            0x28 => Opcode::new(code, 1, 4, AddressingMode::Implied),

            //ROL
            0x2A => Opcode::new(code, 1, 2, AddressingMode::Accumulator),
            0x26 => Opcode::new(code, 2, 5, AddressingMode::ZeroPage),
            0x36 => Opcode::new(code, 2, 6, AddressingMode::ZeroPageX),
            0x2E => Opcode::new(code, 3, 6, AddressingMode::Absolute),
            0x3E => Opcode::new(code, 3, 7, AddressingMode::AbsoluteX),

            //ROR
            0x6A => Opcode::new(code, 1, 2, AddressingMode::Accumulator),
            0x66 => Opcode::new(code, 2, 5, AddressingMode::ZeroPage),
            0x76 => Opcode::new(code, 2, 6, AddressingMode::ZeroPageX),
            0x6E => Opcode::new(code, 3, 6, AddressingMode::Absolute),
            0x7E => Opcode::new(code, 3, 7, AddressingMode::AbsoluteX),

            //RTI
            0x40 => Opcode::new(code, 1, 6, AddressingMode::Implied),
            //RTS
            0x60 => Opcode::new(code, 1, 6, AddressingMode::Implied),

            //SBC
            0xE9 => Opcode::new(code, 2, 2, AddressingMode::Immediate),
            0xE5 => Opcode::new(code, 2, 3, AddressingMode::ZeroPage),
            0xF5 => Opcode::new(code, 2, 4, AddressingMode::ZeroPageX),
            0xED => Opcode::new(code, 3, 4, AddressingMode::Absolute),
            0xFD => Opcode::new(code, 3, 4, AddressingMode::AbsoluteX),
            0xF9 => Opcode::new(code, 3, 4, AddressingMode::AbsoluteY),
            0xE1 => Opcode::new(code, 2, 6, AddressingMode::IndirectX),
            0xF1 => Opcode::new(code, 2, 5, AddressingMode::IndirectY),

            //SEC
            0x38 => Opcode::new(code, 1, 2, AddressingMode::Implied),
            //SED
            0xF8 => Opcode::new(code, 1, 2, AddressingMode::Implied),
            //SEI
            0x78 => Opcode::new(code, 1, 2, AddressingMode::Implied),

            //STA
            0x85 => Opcode::new(code, 2, 3, AddressingMode::ZeroPage),
            0x95 => Opcode::new(code, 2, 4, AddressingMode::ZeroPageX),
            0x8D => Opcode::new(code, 3, 4, AddressingMode::Absolute),
            0x9D => Opcode::new(code, 3, 5, AddressingMode::AbsoluteX),
            0x99 => Opcode::new(code, 3, 5, AddressingMode::AbsoluteY),
            0x81 => Opcode::new(code, 2, 6, AddressingMode::IndirectX),
            0x91 => Opcode::new(code, 2, 6, AddressingMode::IndirectY),

            //STX
            0x86 => Opcode::new(code, 2, 3, AddressingMode::ZeroPage),
            0x96 => Opcode::new(code, 2, 4, AddressingMode::ZeroPageX),
            0x8E => Opcode::new(code, 3, 4, AddressingMode::Absolute),

            //STY
            0x84 => Opcode::new(code, 2, 3, AddressingMode::ZeroPage),
            0x94 => Opcode::new(code, 2, 4, AddressingMode::ZeroPageX),
            0x8C => Opcode::new(code, 3, 4, AddressingMode::Absolute),

            //TAX
            0xAA => Opcode::new(code, 1, 2, AddressingMode::Implied),
            //TAY
            0xA8 => Opcode::new(code, 1, 2, AddressingMode::Implied),
            //TSX
            0xBA => Opcode::new(code, 1, 2, AddressingMode::Implied),
            //TXA
            0x8A => Opcode::new(code, 1, 2, AddressingMode::Implied),
            //TXS
            0x9A => Opcode::new(code, 1, 2, AddressingMode::Implied),
            //TYA
            0x98 => Opcode::new(code, 1, 2, AddressingMode::Implied),

            _ => unreachable!(),
        }
    }
}
//const LDA: Opcode = Opcode::new(0x69, "LDA", 2, 10, AddressingMode::Immediate);