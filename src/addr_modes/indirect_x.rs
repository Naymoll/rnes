use crate::addr_modes::{AddressingMode, Context, Fetched};
use crate::core::memory::Memory;

pub struct IndirectX;

impl AddressingMode for IndirectX {
    fn fetch(&mut self, ctx: &mut Context) -> Fetched {
        let zero_page_offset = ctx.read(ctx.pc) + ctx.x;
        ctx.pc += 1;

        let lo = ctx.read(zero_page_offset);
        let hi = ctx.read(zero_page_offset + 1);

        let address = u16::from_le_bytes([lo, hi]);
        let value = ctx.read(address);

        Fetched::no_cycle(value)
    }
}
