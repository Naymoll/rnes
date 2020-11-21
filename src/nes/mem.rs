//TODO: Дополнительно прочитать про AddressingMode в NES.
// Нужны методы для u16?
#[allow(dead_code)]
pub trait Memory {
    fn read_u8(&self, address: u16) -> u8;
    fn read_u16(&self, address: u16) -> u16;

    fn write_u8(&mut self, address: u16, value: u8);
    fn write_u16(&mut self, address: u16, value: u16);
}
