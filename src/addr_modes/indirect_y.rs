use crate::addr_modes::{AddressingMode, Context, Fetched};
use crate::core::memory::Memory;

pub struct IndirectY;

impl AddressingMode for IndirectY {
    fn fetch(&mut self, ctx: &mut Context) -> Fetched {
        let zero_page_offset = ctx.read(ctx.pc);
        ctx.pc += 1;

        let lo = ctx.read(zero_page_offset);
        let hi = ctx.read(zero_page_offset + 1);

        let mut address = u16::from_le_bytes([lo, hi]);
        address += ctx.y as u16;

        let value = ctx.read(address);
        let [_, addr_hi] = address.to_le_bytes();

        Fetched::some(value, addr_hi != hi)
    }
}
