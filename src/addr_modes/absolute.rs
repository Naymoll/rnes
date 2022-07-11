use crate::addr_modes::{AddressingMode, Context};
use crate::core::memory::Memory;

pub struct Absolute;

impl AddressingMode for Absolute {
    fn address(&mut self, ctx: &mut Context) -> u16 {
        let lo = ctx.read(ctx.pc) as u16;
        ctx.pc += 1;

        let hi = ctx.read(ctx.pc) as u16;
        ctx.pc += 1;

        (hi << 8) | lo
    }
}
