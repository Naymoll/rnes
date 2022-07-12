use crate::addr_modes::{AddressingMode, Context};

pub struct Accum;

impl AddressingMode for Accum {
    fn address(&mut self, ctx: &mut Context) -> u16 {
        unreachable!("this addressing mode shouldn't be used directly")
    }
}
