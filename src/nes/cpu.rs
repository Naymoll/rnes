bitflags! {
    pub struct CpuFlags: u8 {
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

pub struct CPU {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: CpuFlags,
}

#[allow(dead_code)]
impl CPU {
    pub fn new() -> Self {
        CPU {
            program_counter: 0,
            stack_pointer: 0,
            accumulator: 0,
            register_x: 0,
            register_y: 0,
            status: CpuFlags::ONE,
        }
    }

    pub fn execute_commands(&mut self, commands: std::vec::Vec<u8>) {
        loop {
            if (self.program_counter as usize) >= commands.len() {
                return;
            }

            let command = commands[self.program_counter as usize];
            self.program_counter += 1;

            match command {
                0x69 => {
                    self.adc(commands[self.program_counter as usize]);
                    self.program_counter += 1;
                }
                0xA9 => {
                    self.lda(commands[self.program_counter as usize]);
                    self.program_counter += 1;
                }
                0xAA => self.tax(),
                _ => unimplemented!("That opcode unimplemented"),
            }
        }
    }

    //TODO: Заменить инкремент счеткчика данной функцией,
    // в случае, если не нужно переполнение чисел - заменить на saturation_add()
    #[allow(dead_code)]
    fn increment_program_counter(&mut self, value: u16) {
        self.program_counter = self.program_counter.wrapping_add(value);
    }

    fn set_carry_flag(&mut self, value: bool) {
        self.status.set(CpuFlags::CARRY, value);
    }

    fn update_zero_flag(&mut self, value: u8) {
        self.status.set(CpuFlags::ZERO, value == 0);
    }

    fn set_interrupt_disable_flag(&mut self, value: bool) {
        self.status.set(CpuFlags::INTERRUPT_DISABLE, value);
    }

    fn set_decimal_mode_flag(&mut self, value: bool) {
        self.status.set(CpuFlags::DECIMAL_MODE, value);
    }

    fn set_break_command_flag(&mut self, value: bool) {
        self.status.set(CpuFlags::BREAK, value);
    }

    fn set_overflow_flag(&mut self, value: bool) {
        self.status.set(CpuFlags::OVERFLOW, value);
    }

    fn update_negative_flag(&mut self, value: u8) {
        self.status.set(CpuFlags::OVERFLOW, (value & 0b1000_0000) != 0);
    }

    fn set_accumulator(&mut self, value: u8) {
        self.accumulator = value;

        self.update_zero_flag(self.accumulator);
        self.update_negative_flag(self.accumulator);
    }

    //TODO: Возможно есть способ проверки без приведения к u16
    fn add_to_accumulator(&mut self, value: u8) {
        let carry = match self.status.contains(CpuFlags::CARRY) {
            true => 1,
            false => 0,
        };

        let sum: u16 = self.accumulator as u16 + value as u16 + carry;

        match sum > 0xFF {
            true => self.status.insert(CpuFlags::CARRY),
            false => self.status.remove(CpuFlags::CARRY),
        };

        let sum = sum as u8;

        if (sum ^ value) & (sum ^ self.accumulator) & 0x80 != 0 {
            self.status.insert(CpuFlags::OVERFLOW);
        } else {
            self.status.remove(CpuFlags::OVERFLOW);
        }

        self.set_accumulator(sum);
    }

    fn adc(&mut self, value: u8) {
        self.add_to_accumulator(value);
    }

    fn lda(&mut self, value: u8) {
        self.set_accumulator(value);
    }

    fn tax(&mut self) {
        self.register_x = self.accumulator;

        self.update_zero_flag(self.register_x);
        self.update_negative_flag(self.register_x);
    }
}

#[cfg(test)]
mod cpu_test {
    use super::*;

    #[test]
    fn test_lda_negative() {
        let mut cpu = CPU::new();
        cpu.execute_commands(vec![0xA9, 0b1000_0101]);

        assert_eq!(cpu.accumulator, 0b1000_0101);
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
    }

    #[test]
    fn test_lda_zero() {
        let mut cpu = CPU::new();
        cpu.execute_commands(vec![0xA9, 0]);

        assert_eq!(cpu.accumulator, 0);
        assert!(cpu.status.contains(CpuFlags::ZERO));
    }

    #[test]
    fn test_tax_negative() {
        let mut cpu = CPU::new();
        cpu.execute_commands(vec![0xA9, 0b1000_0101, 0xAA]);

        assert_eq!(cpu.register_x, 0b1000_0101);
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
    }

    #[test]
    fn test_tax_zero() {
        let mut cpu = CPU::new();
        cpu.execute_commands(vec![0xA9, 0, 0xAA]);

        assert_eq!(cpu.register_x, 0);
        assert!(cpu.status.contains(CpuFlags::ZERO));
    }

    #[test]
    fn test_adc() {
        let mut cpu = CPU::new();
        cpu.status.insert(CpuFlags::CARRY);
        cpu.execute_commands(vec![0xA9, 20, 0x69, 40]);

        assert_eq!(cpu.accumulator, 61);
    }
}