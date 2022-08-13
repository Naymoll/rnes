use crate::addr_modes::{AddressingMode, Context, Fetched};
use crate::core::memory::Memory;

pub struct ZeroPageY;

impl AddressingMode for ZeroPageY {
    fn fetch(&mut self, ctx: &mut Context) -> Fetched {
        let zero_page_offset = ctx.read(ctx.pc);
        ctx.pc += 1;

        let address = (zero_page_offset + ctx.y) as u16;
        let value = ctx.read(address);

        Fetched::no_cycle(value)
    }
}
