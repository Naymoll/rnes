pub mod absolute;
pub mod accum;
pub mod immediate;
pub mod implied;

use enum_dispatch::enum_dispatch;

use crate::addr_modes::absolute::Absolute;
use crate::addr_modes::accum::Accum;
use crate::addr_modes::immediate::Immediate;
use crate::addr_modes::implied::Implied;

type Context = crate::core::cpu::Cpu;

#[enum_dispatch]
pub trait AddressingMode {
    // TODO: Rename?
    fn address(&mut self, ctx: &mut Context) -> u16;
}

#[enum_dispatch(AddressingMode)]
pub enum AddressingModes {
    Absolute,
    Accum,
    Immediate,
    Implied,
}
