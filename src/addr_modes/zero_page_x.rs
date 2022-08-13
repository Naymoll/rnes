use crate::addr_modes::{AddressingMode, Context, Fetched};
use crate::core::memory::Memory;

pub struct ZeroPageX;

impl AddressingMode for ZeroPageX {
    fn fetch(&mut self, ctx: &mut Context) -> Fetched {
        let zero_page_offset = ctx.read(ctx.pc);
        ctx.pc += 1;

        let address = (zero_page_offset + ctx.x) as u16;
        let value = ctx.read(address);

        Fetched::no_cycle(value)
    }
}
