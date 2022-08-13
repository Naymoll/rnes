use crate::addr_modes::{AddressingMode, Context, Fetched};
use crate::core::memory::Memory;

pub struct AbsoluteX;

impl AddressingMode for AbsoluteX {
    fn fetch(&mut self, ctx: &mut Context) -> Fetched {
        let lo = ctx.read(ctx.pc);
        ctx.pc += 1;

        let hi = ctx.read(ctx.pc);
        ctx.pc += 1;

        let mut address = u16::from_le_bytes([lo, hi]);
        address += ctx.x as u16;

        let value = ctx.read(address);
        let [_, addr_hi] = address.to_le_bytes();

        Fetched::some(value, addr_hi != hi)
    }
}
