pub trait Memory<A>
where
    A: Into<usize>,
{
    const MAX_ADDRESS: usize;

    fn read(&self, address: A) -> u8;
    fn write(&mut self, address: A, value: u8);
}

pub trait Stack<A>: Memory<A>
where
    A: Into<usize>,
{
    const STACK_ADDRESS: u16;
    const STACK_RESET_ADDRESS: u16;

    fn pointer(&self) -> A;
    fn increment_pointer(&mut self);
    fn decrement_pointer(&mut self);

    fn pop(&mut self) -> u8 {
        let address = self.pointer();
        let value = self.read(address);
        self.increment_pointer();

        value
    }

    fn push(&mut self, value: u8) {
        let address = self.pointer();
        self.write(address, value);
        self.decrement_pointer();
    }
}
