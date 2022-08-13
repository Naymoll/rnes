use crate::addr_modes::{AddressingMode, Context, Fetched};
use crate::core::memory::Memory;

pub struct Relative;

impl AddressingMode for Relative {
    fn fetch(&mut self, ctx: &mut Context) -> Fetched {
        let offset = ctx.read(ctx.pc);
        ctx.pc += 1;

        // If bit 7 of the address is 1.
        // Then the address is negative.
        let address = if offset.leading_ones() >= 1 {
            ctx.pc - offset as u16
        } else {
            ctx.pc + offset as u16
        };

        let value = ctx.read(address);
        Fetched::no_cycle(value)
    }
}
