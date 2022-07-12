use crate::addr_modes::{AddressingMode, Context};
use crate::core::memory::Memory;

pub struct Indirect;

impl AddressingMode for Indirect {
    fn address(&mut self, ctx: &mut Context) -> u16 {
        let lo = ctx.read(ctx.pc) as u16;
        ctx.pc += 1;

        let hi = ctx.read(ctx.pc) as u16;
        ctx.pc += 1;

        let pointer = (hi << 8) | lo;
        if lo == 0x00FF {
            // Simulate page boundary hardware bug
            let lo = ctx.read(pointer) as u16;
            let hi = ctx.read(pointer & 0xFF00) as u16;
            (hi << 8) | lo
        } else {
            let lo = ctx.read(pointer) as u16;
            let hi = ctx.read(pointer + 1) as u16;
            (hi << 8) | lo
        }
    }
}
