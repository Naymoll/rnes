use crate::addr_modes::{AddressingMode, Context};
use crate::core::memory::Memory;

pub struct ZeroPage;

impl AddressingMode for ZeroPage {
    fn address(&mut self, ctx: &mut Context) -> u16 {
        let zero_page_offset = ctx.read(ctx.pc) as u16;
        ctx.pc += 1;

        zero_page_offset
    }
}
