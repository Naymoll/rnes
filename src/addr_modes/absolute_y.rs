use crate::addr_modes::{AddressingMode, Context};
use crate::core::memory::Memory;

pub struct AbsoluteY;

impl AddressingMode for AbsoluteY {
    fn address(&mut self, ctx: &mut Context) -> u16 {
        let lo = ctx.read(ctx.pc) as u16;
        ctx.pc += 1;

        let hi = ctx.read(ctx.pc) as u16;
        ctx.pc += 1;

        let mut address = (hi << 8) | lo;
        address += ctx.y as u16;

        // TODO: Add return additional cycle if page is changed
        if (address & 0xFF00) != (hi << 8) {
            eprintln!("Warning: Page is changed. Need additional cycle")
        }

        address
    }
}
