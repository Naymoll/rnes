use crate::addr_modes::{AddressingMode, Context};
use crate::core::memory::Memory;

pub struct IndirectX;

impl AddressingMode for IndirectX {
    fn address(&mut self, ctx: &mut Context) -> u16 {
        let zero_page_offset = ctx.read(ctx.pc) + ctx.x;
        ctx.pc += 1;

        let lo = ctx.read(zero_page_offset as u16);
        let hi = ctx.read((zero_page_offset + 1) as u16);

        u16::from_le_bytes([lo, hi])
    }
}
