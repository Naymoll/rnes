use bitflags::bitflags;

use crate::instructions::Instruction;
use crate::nes::NotFoundError;

const INITIAL_PC: u16 = 0x34;
const INITIAL_SP: u16 = 0xFD;

bitflags! {
    pub struct Flags: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL_MODE      = 0b00001000;
        const BREAK             = 0b00010000;
        const ONE               = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIVE          = 0b10000000;
    }
}

#[derive(Debug)]
pub struct CPU {
    /// Program counter
    pc: u16,
    /// Stack pointer
    sp: u16,
    /// Accumulator
    acc: u8,
    /// X index
    x: u8,
    /// Y index
    y: u8,
    /// CPU flags
    flags: Flags,
    // TODO: change to proper type
    memory: [u8; 65536],
}

impl CPU {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.sp = self.sp - 3;
        self.flags.set(Flags::INTERRUPT_DISABLE, true);
        todo!("APU: https://www.nesdev.org/wiki/CPU_power_up_state")
    }

    // TODO: Change fn name
    pub fn execute(&mut self, opcodes: &[u8]) {
        todo!("CPU code execution")
    }

    pub fn decode(&self, opcode: u8) -> Result<Box<dyn Instruction>, NotFoundError> {
        match opcode {
            _ => Err(NotFoundError::new(opcode)),
        }
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            pc: INITIAL_PC,
            sp: INITIAL_SP,
            acc: 0,
            x: 0,
            y: 0,
            flags: Flags::INTERRUPT_DISABLE | Flags::BREAK | Flags::ONE, // TODO: Recheck
            memory: [0; 65536],
        }
    }
}
