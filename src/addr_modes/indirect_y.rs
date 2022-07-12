use crate::addr_modes::{AddressingMode, Context};
use crate::core::memory::Memory;

pub struct IndirectY;

impl AddressingMode for IndirectY {
    fn address(&mut self, ctx: &mut Context) -> u16 {
        let zero_page_offset = ctx.read(ctx.pc);
        ctx.pc += 1;

        let lo = ctx.read(zero_page_offset as u16);
        let hi = ctx.read((zero_page_offset + 1) as u16);

        let mut address = u16::from_le_bytes([lo, hi]);
        address += ctx.y as u16;

        let [_, addr_hi] = address.to_le_bytes();
        if addr_hi != hi {
            // TODO: Add return additional cycle if page is changed
            eprintln!("Warning: Page is changed. Need additional cycle");
            address
        } else {
            address
        }
    }
}
