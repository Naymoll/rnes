use crate::addr_modes::{AddressingMode, Context, Fetched};

pub struct Implied;

impl AddressingMode for Implied {
    fn fetch(&mut self, ctx: &mut Context) -> Fetched {
        Fetched::none()
    }
}
