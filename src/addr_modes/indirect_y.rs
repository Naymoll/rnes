use crate::addr_modes::{AddressingMode, Context};
use crate::core::memory::Memory;

pub struct IndirectY;

impl AddressingMode for IndirectY {
    fn address(&mut self, ctx: &mut Context) -> u16 {
        let pointer = ctx.read(ctx.pc);
        ctx.pc += 1;

        let lo = ctx.read(pointer as u16) as u16;
        let hi = ctx.read((pointer + 1) as u16) as u16;

        let mut address = (hi << 8) | lo;
        address += ctx.y as u16;

        // TODO: Add return additional cycle if page is changed
        if (address & 0xFF00) != (hi << 8) {
            eprintln!("Warning: Page is changed. Need additional cycle")
        }

        address
    }
}
