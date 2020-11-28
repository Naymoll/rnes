pub trait Memory {
    fn read_u8(&self, address: u16) -> u8;
    fn read_u16(&self, address: u16) -> u16;

    fn write_u8(&mut self, address: u16, value: u8);
    fn write_u16(&mut self, address: u16, value: u16);
}

pub trait Stack {
    fn pop_u8(&mut self) -> u8;
    fn pop_u16(&mut self) -> u16;

    fn push_u8(&mut self, value: u8);
    fn push_u16(&mut self, value: u16);
}
