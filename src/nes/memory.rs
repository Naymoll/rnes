pub trait Memory<V, A> {
    fn read(&self, address: A) -> V;
    fn write(&mut self, address: A, value: V);
}

pub trait Stack<V, A>: Memory<V, A> {
    const STACK_ADDRESS: A;
    const STACK_RESET_ADDRESS: A;

    fn pointer(&self) -> A;
    fn increment_pointer(&mut self);
    fn decrement_pointer(&mut self);

    fn pop(&mut self) -> V {
        let value = self.read(self.pointer());
        self.increment_pointer();

        value
    }

    fn pull(&mut self, value: V) {
        self.write(self.pointer(), value);
        self.decrement_pointer();
    }
}