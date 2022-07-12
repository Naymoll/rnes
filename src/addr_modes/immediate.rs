use crate::addr_modes::{AddressingMode, Context};

pub struct Immediate;

impl AddressingMode for Immediate {
    fn address(&mut self, ctx: &mut Context) -> u16 {
        let address = ctx.pc;
        ctx.pc += 1;

        address
    }
}
