use crate::addr_modes::{AddressingMode, Context};

pub struct Accum;

impl AddressingMode for Accum {
    fn address(&mut self, ctx: &mut Context) -> u16 {
        (ctx.acc as u16) << 8
    }
}
