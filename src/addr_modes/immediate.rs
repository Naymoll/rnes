use crate::addr_modes::{AddressingMode, Context, Fetched};
use crate::core::memory::Memory;

pub struct Immediate;

impl AddressingMode for Immediate {
    fn fetch(&mut self, ctx: &mut Context) -> Fetched {
        let address = ctx.pc;
        ctx.pc += 1;

        let value = ctx.read(address);
        Fetched::no_cycle(value)
    }
}
