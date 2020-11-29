#[derive(PartialEq, Eq)]
pub enum InterruptType {
    Reset,
    NMI,
    IRQ,
    BRK,
}

pub struct Interrupt {
    pub int_type: InterruptType,
    pub vec_addr: u16,
}

pub const RESET_INT: Interrupt = Interrupt {
    int_type: InterruptType::Reset,
    vec_addr: 0xFFFC,
};

pub const NMI_RESET: Interrupt = Interrupt {
    int_type: InterruptType::NMI,
    vec_addr: 0xFFFA,
};

pub const IRQ_INT: Interrupt = Interrupt {
    int_type: InterruptType::IRQ,
    vec_addr: 0xFFFE,
};

pub const BRK_INT: Interrupt = Interrupt {
    int_type: InterruptType::BRK,
    vec_addr: 0xFFFE,
};