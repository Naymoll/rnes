pub trait Memory {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

pub trait Stack: Memory {
    const STACK_ADDRESS: u16;
    const STACK_RESET_ADDRESS: u16;

    fn pointer(&self) -> u16;
    fn increment_pointer(&mut self);
    fn decrement_pointer(&mut self);

    fn pop(&mut self) -> u8 {
        let value = self.read(self.pointer());
        self.increment_pointer();

        value
    }

    fn push(&mut self, value: u8) {
        self.write(self.pointer(), value);
        self.decrement_pointer();
    }
}
