use crate::addr_modes::{AddressingMode, Context};
use crate::core::memory::Memory;

pub struct ZeroPageY;

impl AddressingMode for ZeroPageY {
    fn address(&mut self, ctx: &mut Context) -> u16 {
        let zero_page_offset = ctx.read(ctx.pc);
        ctx.pc += 1;

        (zero_page_offset + ctx.y) as u16
    }
}
