pub struct Context;

pub trait Instruction: TryFrom<u8> {
    // TODO: Replace ctx with proper type
    fn execute(&mut self, ctx: &mut Context);

    // Getters
    fn opcode(&self) -> u8;
    fn len(&self) -> u8;
    fn cycle(&self) -> u8;
}