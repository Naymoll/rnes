pub mod absolute;
pub mod absolute_x;
pub mod absolute_y;
pub mod accum;
pub mod immediate;
pub mod implied;
pub mod indirect;
pub mod indirect_x;
pub mod indirect_y;
pub mod relative;
pub mod zero_page;
pub mod zero_page_x;
pub mod zero_page_y;

use enum_dispatch::enum_dispatch;

use crate::addr_modes::absolute::Absolute;
use crate::addr_modes::absolute_x::AbsoluteX;
use crate::addr_modes::absolute_y::AbsoluteY;
use crate::addr_modes::accum::Accum;
use crate::addr_modes::immediate::Immediate;
use crate::addr_modes::implied::Implied;
use crate::addr_modes::indirect::Indirect;
use crate::addr_modes::indirect_x::IndirectX;
use crate::addr_modes::indirect_y::IndirectY;
use crate::addr_modes::relative::Relative;
use crate::addr_modes::zero_page::ZeroPage;
use crate::addr_modes::zero_page_x::ZeroPageX;
use crate::addr_modes::zero_page_y::ZeroPageY;

type Context = crate::core::cpu::Cpu;

#[enum_dispatch]
pub trait AddressingMode {
    // TODO: Rename?
    fn address(&mut self, ctx: &mut Context) -> u16;
}

#[enum_dispatch(AddressingMode)]
pub enum AddressingModes {
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Accum,
    Immediate,
    Implied,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
}
