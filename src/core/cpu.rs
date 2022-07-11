use crate::core::memory::{Memory, Stack};
use bitflags::bitflags;

use crate::core::NotFoundError;
use crate::instructions::Instruction;

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
// TODO: Change fields visibility
pub struct Cpu {
    /// Program counter
    pub pc: u16,
    /// Stack pointer
    pub sp: u16,
    /// Accumulator
    pub acc: u8,
    /// X index
    pub x: u8,
    /// Y index
    pub y: u8,
    /// CPU flags
    pub flags: Flags,
    // TODO: change to proper type
    pub memory: [u8; 65536],
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
    }

    // TODO: Recheck
    pub fn reset(&mut self) {
        self.sp -= 3;
        self.flags.insert(Flags::INTERRUPT_DISABLE);
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

impl Default for Cpu {
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

impl Memory for Cpu {
    fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}

impl Stack for Cpu {
    const STACK_ADDRESS: u16 = INITIAL_SP;
    const STACK_RESET_ADDRESS: u16 = INITIAL_SP - 3;

    fn pointer(&self) -> u16 {
        self.pc
    }

    fn increment_pointer(&mut self) {
        self.pc += 1;
    }

    fn decrement_pointer(&mut self) {
        self.pc -= 1;
    }
}
