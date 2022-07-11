use thiserror::Error;

pub mod cpu;
pub mod memory;

#[derive(Error, Debug)]
#[error("opcode {opcode:#X} not found")]
pub struct NotFoundError {
    opcode: u8,
}

impl NotFoundError {
    pub fn new(opcode: u8) -> Self {
        Self { opcode }
    }
}
