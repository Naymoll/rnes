use crate::addr_modes::{AddressingMode, Context};
use crate::core::memory::Memory;

pub struct IndirectX;

impl AddressingMode for IndirectX {
    fn address(&mut self, ctx: &mut Context) -> u16 {
        let pointer = ctx.read(ctx.pc) + ctx.x;
        ctx.pc += 1;

        let lo = ctx.read(pointer as u16) as u16;
        let hi = ctx.read((pointer + 1) as u16) as u16;

        (hi << 8) | lo
    }
}
