use crate::addr_modes::{AddressingMode, Context, Fetched};
use crate::core::memory::Memory;

pub struct Indirect;

impl AddressingMode for Indirect {
    fn fetch(&mut self, ctx: &mut Context) -> Fetched {
        let ptr_lo = ctx.read(ctx.pc);
        ctx.pc += 1;

        let ptr_hi = ctx.read(ctx.pc);
        ctx.pc += 1;

        let pointer = u16::from_le_bytes([ptr_lo, ptr_hi]);

        let lo = ctx.read(pointer);
        let hi = if ptr_lo == u8::MAX {
            // Simulate page boundary hardware bug.
            // More: https://www.nesdev.org/6502bugs.txt

            let bab_pointer = u16::from_le_bytes([u8::MIN, ptr_hi]);
            ctx.read(bab_pointer)
        } else {
            ctx.read(pointer + 1)
        };

        let address = u16::from_le_bytes([lo, hi]);
        let value = ctx.read(address);

        Fetched::no_cycle(value)
    }
}
