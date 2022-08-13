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

pub use crate::addr_modes::absolute::Absolute;
pub use crate::addr_modes::absolute_x::AbsoluteX;
pub use crate::addr_modes::absolute_y::AbsoluteY;
pub use crate::addr_modes::accum::Accum;
pub use crate::addr_modes::immediate::Immediate;
pub use crate::addr_modes::implied::Implied;
pub use crate::addr_modes::indirect::Indirect;
pub use crate::addr_modes::indirect_x::IndirectX;
pub use crate::addr_modes::indirect_y::IndirectY;
pub use crate::addr_modes::relative::Relative;
pub use crate::addr_modes::zero_page::ZeroPage;
pub use crate::addr_modes::zero_page_x::ZeroPageX;
pub use crate::addr_modes::zero_page_y::ZeroPageY;

type Context = crate::core::cpu::Cpu;

#[derive(Debug, Default, Copy, Clone)]
pub struct Fetched {
    value: Option<u8>,
    add_cycle: bool,
}

#[allow(dead_code)]
impl Fetched {
    pub fn some(value: u8, add_cycle: bool) -> Self {
        Self {
            value: Some(value),
            add_cycle,
        }
    }

    pub fn no_cycle(value: u8) -> Self {
        Self {
            value: Some(value),
            add_cycle: false,
        }
    }

    pub fn none() -> Self {
        Self::default()
    }

    pub fn value(&self) -> Option<u8> {
        self.value
    }

    pub fn add_cycle(&self) -> bool {
        self.add_cycle
    }

    pub fn is_some(&self) -> bool {
        self.value.is_some()
    }

    pub fn is_none(&self) -> bool {
        self.value.is_none()
    }
}

#[enum_dispatch]
pub trait AddressingMode {
    fn fetch(&mut self, ctx: &mut Context) -> Fetched;
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
