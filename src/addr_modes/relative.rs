use crate::addr_modes::{AddressingMode, Context};
use crate::core::memory::Memory;

pub struct Relative;

impl AddressingMode for Relative {
    fn address(&mut self, ctx: &mut Context) -> u16 {
        let address = ctx.read(ctx.pc) as u8;
        ctx.pc += 1;

        // If bit 7 of the address is 1.
        // Then the address is negative.
        if address.leading_ones() >= 1 {
            ctx.pc - address as u16
        } else {
            ctx.pc + address as u16
        }
    }
}
