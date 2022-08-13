use crate::addr_modes::{AddressingMode, Context, Fetched};
use crate::core::memory::Memory;

pub struct Absolute;

impl AddressingMode for Absolute {
    fn fetch(&mut self, ctx: &mut Context) -> Fetched {
        let lo = ctx.read(ctx.pc);
        ctx.pc += 1;

        let hi = ctx.read(ctx.pc);
        ctx.pc += 1;

        let address = u16::from_le_bytes([lo, hi]);
        let value = ctx.read(address);

        Fetched::no_cycle(value)
    }
}
