use crate::addr_modes::{AddressingMode, Context};
use crate::core::memory::Memory;

pub struct Relative;

impl AddressingMode for Relative {
    fn address(&mut self, ctx: &mut Context) -> u16 {
        let offset = ctx.read(ctx.pc);
        ctx.pc += 1;

        // If bit 7 of the address is 1.
        // Then the address is negative.
        if offset.leading_ones() >= 1 {
            ctx.pc - offset as u16
        } else {
            ctx.pc + offset as u16
        }
    }
}
