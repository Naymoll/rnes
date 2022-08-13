use crate::addr_modes::{AddressingMode, Context, Fetched};

pub struct Accum;

impl AddressingMode for Accum {
    fn fetch(&mut self, ctx: &mut Context) -> Fetched {
        Fetched::no_cycle(ctx.acc)
    }
}
