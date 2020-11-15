use crate::nes::cpu::AddressingMode;

#[allow(dead_code)]
pub struct Instruction {
    pub opcode: u8,
    pub len: u8,
    pub cycle: u8,
    pub addressing_mode: AddressingMode
}

#[allow(dead_code)]
impl Instruction {
    pub const  fn new(opcode: u8, len: u8, cycle: u8, addressing_mode: AddressingMode) -> Self {
        Instruction {
            opcode,
            len,
            cycle,
            addressing_mode,
        }
    }

    pub fn from_opcode(opcode: u8) -> Self {
        match opcode {
            //ADC
            0x69 => Instruction::new(opcode, 2, 2, AddressingMode::Immediate),
            0x65 => Instruction::new(opcode, 2, 3, AddressingMode::ZeroPage),
            0x75 => Instruction::new(opcode, 2, 4, AddressingMode::ZeroPageX),
            0x6D => Instruction::new(opcode, 3, 4, AddressingMode::Absolute),
            0x7D => Instruction::new(opcode, 3, 4, AddressingMode::AbsoluteX),
            0x79 => Instruction::new(opcode, 3, 4, AddressingMode::AbsoluteY),
            0x61 => Instruction::new(opcode, 2, 6, AddressingMode::IndirectX),
            0x71 => Instruction::new(opcode, 2, 5, AddressingMode::IndirectY),

            //AND
            0x29 => Instruction::new(opcode, 2, 2, AddressingMode::Immediate),
            0x25 => Instruction::new(opcode, 2, 3, AddressingMode::ZeroPage),
            0x35 => Instruction::new(opcode, 2, 4, AddressingMode::ZeroPageX),
            0x2D => Instruction::new(opcode, 3, 4, AddressingMode::Absolute),
            0x3D => Instruction::new(opcode, 3, 4, AddressingMode::AbsoluteX),
            0x39 => Instruction::new(opcode, 3, 4, AddressingMode::AbsoluteY),
            0x21 => Instruction::new(opcode, 2, 6, AddressingMode::IndirectX),
            0x31 => Instruction::new(opcode, 2, 5, AddressingMode::IndirectY),

            //ASL
            0x0A => Instruction::new(opcode, 1, 2, AddressingMode::Accumulator),
            0x06 => Instruction::new(opcode, 2, 5, AddressingMode::ZeroPage),
            0x16 => Instruction::new(opcode, 2, 6, AddressingMode::ZeroPageX),
            0x0E => Instruction::new(opcode, 3, 6, AddressingMode::Absolute),
            0x1E => Instruction::new(opcode, 3, 7, AddressingMode::AbsoluteX),

            //BCC
            0x90 => Instruction::new(opcode, 2, 2, AddressingMode::Relative),
            //BCS
            0xB0 => Instruction::new(opcode, 2, 2, AddressingMode::Relative),
            //BEQ
            0xF0 => Instruction::new(opcode, 2, 2, AddressingMode::Relative),

            //BIT
            0x24 => Instruction::new(opcode, 2, 3, AddressingMode::ZeroPage),
            0x2C => Instruction::new(opcode, 3, 4, AddressingMode::Absolute),

            //BMI
            0x30 => Instruction::new(opcode, 2, 2, AddressingMode::Relative),
            //BNE
            0xD0 => Instruction::new(opcode, 2, 2, AddressingMode::Relative),
            //BPL
            0x10 => Instruction::new(opcode, 2, 2, AddressingMode::Relative),
            //BRK
            0x00 => Instruction::new(opcode, 1, 7, AddressingMode::Implied),
            //BVC
            0x50 => Instruction::new(opcode, 2, 2, AddressingMode::Relative),
            //BVS
            0x70 => Instruction::new(opcode, 2, 2, AddressingMode::Relative),

            //CLC
            0x18 => Instruction::new(opcode, 1, 2, AddressingMode::Implied),
            //CLD
            0xD8 => Instruction::new(opcode, 1, 2, AddressingMode::Implied),
            //CLI
            0x58 => Instruction::new(opcode, 1, 2, AddressingMode::Implied),
            //CLV
            0xB8 => Instruction::new(opcode, 1, 2, AddressingMode::Implied),

            //CMP
            0xC9 => Instruction::new(opcode, 2, 2, AddressingMode::Immediate),
            0xC5 => Instruction::new(opcode, 2, 3, AddressingMode::ZeroPage),
            0xD5 => Instruction::new(opcode, 2, 4, AddressingMode::ZeroPageX),
            0xCD => Instruction::new(opcode, 3, 4, AddressingMode::Absolute),
            0xDD => Instruction::new(opcode, 3, 4, AddressingMode::AbsoluteX),
            0xD9 => Instruction::new(opcode, 3, 4, AddressingMode::AbsoluteY),
            0xC1 => Instruction::new(opcode, 2, 6, AddressingMode::IndirectX),
            0xD1 => Instruction::new(opcode, 2, 5, AddressingMode::IndirectY),

            //CPX
            0xE0 => Instruction::new(opcode, 2, 2, AddressingMode::Immediate),
            0xE4 => Instruction::new(opcode, 2, 3, AddressingMode::ZeroPage),
            0xEC => Instruction::new(opcode, 3, 4, AddressingMode::Absolute),

            //CPY
            0xC0 => Instruction::new(opcode, 2, 2, AddressingMode::Immediate),
            0xC4 => Instruction::new(opcode, 2, 3, AddressingMode::ZeroPage),
            0xCC => Instruction::new(opcode, 3, 4, AddressingMode::Absolute),

            //DEC
            0xC6 => Instruction::new(opcode, 2, 5, AddressingMode::ZeroPage),
            0xD6 => Instruction::new(opcode, 2, 6, AddressingMode::ZeroPageX),
            0xCE => Instruction::new(opcode, 3, 6, AddressingMode::Absolute),
            0xDE => Instruction::new(opcode, 3, 7, AddressingMode::AbsoluteX),

            //DEX
            0xCA => Instruction::new(opcode, 1, 2, AddressingMode::Implied),
            //DEY
            0x88 => Instruction::new(opcode, 1, 2, AddressingMode::Implied),

            //EOR
            0x49 => Instruction::new(opcode, 2, 2, AddressingMode::Immediate),
            0x45 => Instruction::new(opcode, 2, 3, AddressingMode::ZeroPage),
            0x55 => Instruction::new(opcode, 2, 4, AddressingMode::ZeroPageX),
            0x4D => Instruction::new(opcode, 3, 4, AddressingMode::Absolute),
            0x5D => Instruction::new(opcode, 3, 4, AddressingMode::AbsoluteX),
            0x59 => Instruction::new(opcode, 3, 4, AddressingMode::AbsoluteY),
            0x41 => Instruction::new(opcode, 2, 6, AddressingMode::IndirectX),
            0x51 => Instruction::new(opcode, 2, 5, AddressingMode::IndirectY),

            //INC
            0xE6 => Instruction::new(opcode, 2, 5, AddressingMode::ZeroPage),
            0xF6 => Instruction::new(opcode, 2, 6, AddressingMode::ZeroPageX),
            0xEE => Instruction::new(opcode, 3, 6, AddressingMode::Absolute),
            0xFE => Instruction::new(opcode, 3, 7, AddressingMode::AbsoluteX),

            //INX
            0xE8 => Instruction::new(opcode, 1, 2, AddressingMode::Implied),
            //INY
            0xC8 => Instruction::new(opcode, 1, 2, AddressingMode::Implied),

            //JMP
            0x4C => Instruction::new(opcode, 3, 3, AddressingMode::Absolute),
            0x6C => Instruction::new(opcode, 3, 5, AddressingMode::Indirect),

            //JSR
            0x20 => Instruction::new(opcode, 3, 6, AddressingMode::Absolute),

            //LDA
            0xA9 => Instruction::new(opcode, 2, 2, AddressingMode::Immediate),
            0xA5 => Instruction::new(opcode, 2, 3, AddressingMode::ZeroPage),
            0xB5 => Instruction::new(opcode, 2, 4, AddressingMode::ZeroPageX),
            0xAD => Instruction::new(opcode, 3, 4, AddressingMode::Absolute),
            0xBD => Instruction::new(opcode, 3, 4, AddressingMode::AbsoluteX),
            0xB9 => Instruction::new(opcode, 3, 4, AddressingMode::AbsoluteY),
            0xA1 => Instruction::new(opcode, 2, 6, AddressingMode::IndirectX),
            0xB1 => Instruction::new(opcode, 2, 5, AddressingMode::IndirectY),

            //LDX
            0xA2 => Instruction::new(opcode, 2, 2, AddressingMode::Immediate),
            0xA6 => Instruction::new(opcode, 2, 3, AddressingMode::ZeroPage),
            0xB6 => Instruction::new(opcode, 2, 4, AddressingMode::ZeroPageY),
            0xAE => Instruction::new(opcode, 3, 4, AddressingMode::Absolute),
            0xBE => Instruction::new(opcode, 3, 4, AddressingMode::AbsoluteY),

            //LDY
            0xA0 => Instruction::new(opcode, 2, 2, AddressingMode::Immediate),
            0xA4 => Instruction::new(opcode, 2, 3, AddressingMode::ZeroPage),
            0xB4 => Instruction::new(opcode, 2, 4, AddressingMode::ZeroPageX),
            0xAC => Instruction::new(opcode, 3, 4, AddressingMode::Absolute),
            0xBC => Instruction::new(opcode, 3, 4, AddressingMode::AbsoluteX),

            //LSR
            0x4A => Instruction::new(opcode, 1, 2, AddressingMode::Accumulator),
            0x46 => Instruction::new(opcode, 2, 5, AddressingMode::ZeroPage),
            0x56 => Instruction::new(opcode, 2, 6, AddressingMode::ZeroPageX),
            0x4E => Instruction::new(opcode, 3, 6, AddressingMode::Absolute),
            0x5E => Instruction::new(opcode, 3, 7, AddressingMode::AbsoluteX),

            //NOP
            0xEA => Instruction::new(opcode, 1, 2, AddressingMode::Implied),

            //ORA
            0x09 => Instruction::new(opcode, 2, 2, AddressingMode::Immediate),
            0x05 => Instruction::new(opcode, 2, 3, AddressingMode::ZeroPage),
            0x15 => Instruction::new(opcode, 2, 4, AddressingMode::ZeroPageX),
            0x0D => Instruction::new(opcode, 3, 4, AddressingMode::Absolute),
            0x1D => Instruction::new(opcode, 3, 4, AddressingMode::AbsoluteX),
            0x19 => Instruction::new(opcode, 3, 4, AddressingMode::AbsoluteY),
            0x01 => Instruction::new(opcode, 2, 6, AddressingMode::IndirectX),
            0x11 => Instruction::new(opcode, 2, 5, AddressingMode::IndirectY),

            //PHA
            0x48 => Instruction::new(opcode, 1, 3, AddressingMode::Implied),
            //PHP
            0x08 => Instruction::new(opcode, 1, 3, AddressingMode::Implied),
            //PLA
            0x68 => Instruction::new(opcode, 1, 4, AddressingMode::Implied),
            //PLP
            0x28 => Instruction::new(opcode, 1, 4, AddressingMode::Implied),

            //ROL
            0x2A => Instruction::new(opcode, 1, 2, AddressingMode::Accumulator),
            0x26 => Instruction::new(opcode, 2, 5, AddressingMode::ZeroPage),
            0x36 => Instruction::new(opcode, 2, 6, AddressingMode::ZeroPageX),
            0x2E => Instruction::new(opcode, 3, 6, AddressingMode::Absolute),
            0x3E => Instruction::new(opcode, 3, 7, AddressingMode::AbsoluteX),

            //ROR
            0x6A => Instruction::new(opcode, 1, 2, AddressingMode::Accumulator),
            0x66 => Instruction::new(opcode, 2, 5, AddressingMode::ZeroPage),
            0x76 => Instruction::new(opcode, 2, 6, AddressingMode::ZeroPageX),
            0x6E => Instruction::new(opcode, 3, 6, AddressingMode::Absolute),
            0x7E => Instruction::new(opcode, 3, 7, AddressingMode::AbsoluteX),

            //RTI
            0x40 => Instruction::new(opcode, 1, 6, AddressingMode::Implied),
            //RTS
            0x60 => Instruction::new(opcode, 1, 6, AddressingMode::Implied),

            //SBC
            0xE9 => Instruction::new(opcode, 2, 2, AddressingMode::Immediate),
            0xE5 => Instruction::new(opcode, 2, 3, AddressingMode::ZeroPage),
            0xF5 => Instruction::new(opcode, 2, 4, AddressingMode::ZeroPageX),
            0xED => Instruction::new(opcode, 3, 4, AddressingMode::Absolute),
            0xFD => Instruction::new(opcode, 3, 4, AddressingMode::AbsoluteX),
            0xF9 => Instruction::new(opcode, 3, 4, AddressingMode::AbsoluteY),
            0xE1 => Instruction::new(opcode, 2, 6, AddressingMode::IndirectX),
            0xF1 => Instruction::new(opcode, 2, 5, AddressingMode::IndirectY),

            //SEC
            0x38 => Instruction::new(opcode, 1, 2, AddressingMode::Implied),
            //SED
            0xF8 => Instruction::new(opcode, 1, 2, AddressingMode::Implied),
            //SEI
            0x78 => Instruction::new(opcode, 1, 2, AddressingMode::Implied),

            //STA
            0x85 => Instruction::new(opcode, 2, 3, AddressingMode::ZeroPage),
            0x95 => Instruction::new(opcode, 2, 4, AddressingMode::ZeroPageX),
            0x8D => Instruction::new(opcode, 3, 4, AddressingMode::Absolute),
            0x9D => Instruction::new(opcode, 3, 5, AddressingMode::AbsoluteX),
            0x99 => Instruction::new(opcode, 3, 5, AddressingMode::AbsoluteY),
            0x81 => Instruction::new(opcode, 2, 6, AddressingMode::IndirectX),
            0x91 => Instruction::new(opcode, 2, 6, AddressingMode::IndirectY),

            //STX
            0x86 => Instruction::new(opcode, 2, 3, AddressingMode::ZeroPage),
            0x96 => Instruction::new(opcode, 2, 4, AddressingMode::ZeroPageX),
            0x8E => Instruction::new(opcode, 3, 4, AddressingMode::Absolute),

            //STY
            0x84 => Instruction::new(opcode, 2, 3, AddressingMode::ZeroPage),
            0x94 => Instruction::new(opcode, 2, 4, AddressingMode::ZeroPageX),
            0x8C => Instruction::new(opcode, 3, 4, AddressingMode::Absolute),

            //TAX
            0xAA => Instruction::new(opcode, 1, 2, AddressingMode::Implied),
            //TAY
            0xA8 => Instruction::new(opcode, 1, 2, AddressingMode::Implied),
            //TSX
            0xBA => Instruction::new(opcode, 1, 2, AddressingMode::Implied),
            //TXA
            0x8A => Instruction::new(opcode, 1, 2, AddressingMode::Implied),
            //TXS
            0x9A => Instruction::new(opcode, 1, 2, AddressingMode::Implied),
            //TYA
            0x98 => Instruction::new(opcode, 1, 2, AddressingMode::Implied),

            _ => unreachable!(),
        }
    }
}
//const LDA: Opcode = Opcode::new(0x69, "LDA", 2, 10, AddressingMode::Immediate);