use thiserror::Error;

pub struct Context;

pub trait Instruction: TryFrom<u8> {
    // TODO: Replace ctx with proper type
    fn execute(&mut self, ctx: &mut Context);

    // Getters
    fn opcode(&self) -> u8;
    fn len(&self) -> u8;
    fn cycle(&self) -> u8;
}

#[derive(Error, Debug)]
#[error("invalid opcode (expected {expected}, found {found})")]
pub struct InvalidOpcodeError {
    expected: u8,
    found: u8,
}

impl InvalidOpcodeError {
    pub fn new(expected: u8, found: u8) -> Self {
        Self { expected, found }
    }
}
