use crate::addr_modes::{AddressingMode, Context, Fetched};
use crate::core::memory::Memory;

pub struct ZeroPage;

impl AddressingMode for ZeroPage {
    fn fetch(&mut self, ctx: &mut Context) -> Fetched {
        let zero_page_offset = ctx.read(ctx.pc) as u16;
        ctx.pc += 1;

        let value = ctx.read(zero_page_offset);
        Fetched::no_cycle(value)
    }
}
