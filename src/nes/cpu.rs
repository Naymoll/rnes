use bitflags::bitflags;

bitflags! {
    pub struct Flags: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL_MODE      = 0b00001000;
        const BREAK             = 0b00010000;
        const ONE               = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIVE          = 0b10000000;
    }
}

#[derive(Debug)]
pub struct CPU {
    /// Program counter
    pc: u16,
    /// Stack pointer
    sp: u16,
    /// Accumulator
    acc: u8,
    /// X index
    x: u8,
    /// Y index
    y: u8,
    /// CPU flags
    flags: Flags,
}
