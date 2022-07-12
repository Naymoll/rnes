use crate::addr_modes::{AddressingMode, Context};
use crate::core::memory::Memory;

pub struct ZeroPageX;

impl AddressingMode for ZeroPageX {
    fn address(&mut self, ctx: &mut Context) -> u16 {
        let address = ctx.read(ctx.pc) as u16;
        ctx.pc += 1;

        address + ctx.x as u16
    }
}
