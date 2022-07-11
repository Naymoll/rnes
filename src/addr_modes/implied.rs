use crate::addr_modes::{AddressingMode, Context};

pub struct Implied;

impl AddressingMode for Implied {
    fn address(&mut self, ctx: &mut Context) -> u16 {
        unreachable!("this addressing mode shouldn't be used directly")
    }
}
