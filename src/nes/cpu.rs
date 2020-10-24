use crate::nes::opcodes;

bitflags! {
    pub struct CpuFlags: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL_MODE      = 0b00001000;
        const BREAK             = 0b00010000;
        const ONE               = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIVE           = 0b10000000;
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
                0x_A9 => {
                    self.lda(commands[self.program_counter as usize]);
                    self.program_counter += 1;
                }
                0x_AA => self.tax(),
                _ => unimplemented!("That opcode unimplemented"),
            }
        }
    }

    //TODO: Заменить инкремент счеткиа данной функцией, прочитать про переполнение на NES,
    // в случае, если не нужно переполнение чисел - заменить на saturation_add()
    #[allow(dead_code)]
    fn increment_program_counter(&mut self, value: u16) {
        self.program_counter = self.program_counter.wrapping_add(value);
    }

    //TODO: Доделать реализацию
    fn update_carry_flag(&mut self, value: u8) {
        let is_carry: bool = true;
        match is_carry {
            true => self.status.insert(CpuFlags::CARRY),
            false => self.status.remove(CpuFlags::CARRY),
        };
    }

    fn update_zero_flag(&mut self, value: u8) {
        match value == 0 {
            true => self.status.insert(CpuFlags::ZERO),
            false => self.status.remove(CpuFlags::ZERO),
        };
    }

    fn update_interrupt_disable_flag(&mut self, bit: bool) {
        match bit {
            true => self.status.insert(CpuFlags::INTERRUPT_DISABLE),
            false => self.status.remove(CpuFlags::INTERRUPT_DISABLE),
        };
    }

    fn set_decimal_mode_flag(&mut self, bit: bool) {
        match bit {
            true => self.status.insert(CpuFlags::DECIMAL_MODE),
            false => self.status.remove(CpuFlags::DECIMAL_MODE),
        };
    }

    fn set_break_command_flag(&mut self, bit: bool) {
        match bit {
            true => self.status.insert(CpuFlags::BREAK),
            false => self.status.remove(CpuFlags::BREAK),
        };
    }

    //TODO: Доделать реализацию
    fn update_overflow_flag(&mut self, value: u8) {
        match true {
            true => self.status.insert(CpuFlags::OVERFLOW),
            false => self.status.remove(CpuFlags::OVERFLOW),
        };
    }

    fn update_negative_flag(&mut self, value: u8) {
        let is_negative = (value & 0b_1000_0000) != 0;

        match is_negative {
            true => self.status.insert(CpuFlags::NEGATIVE),
            false => self.status.remove(CpuFlags::NEGATIVE),
        };
    }

    //TODO: Возможно стоит поменять сложение, доделать carry и overflow флаги
    fn adc(&mut self, value: u8) {
        let carry_bit = match self.status.contains(CpuFlags::CARRY) {
            true => 1,
            false => 0,
        };

        self.accumulator = self.accumulator.wrapping_add(value);
        self.accumulator = self.accumulator.wrapping_add(carry_bit);

        //self.update_carry_flag(self.accumulator);
        self.update_zero_flag(self.accumulator);
        //self.update_overflow_flag();
        self.update_negative_flag(self.accumulator);
    }

    fn lda(&mut self, value: u8) {
        self.accumulator = value;

        self.update_zero_flag(self.accumulator);
        self.update_negative_flag(self.accumulator);
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
        cpu.execute_commands(vec![0x_A9, 0b_1000_0101]);

        assert_eq!(cpu.accumulator, 0b_1000_0101);
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
    }

    #[test]
    fn test_lda_zero() {
        let mut cpu = CPU::new();
        cpu.execute_commands(vec![0x_A9, 0]);

        assert_eq!(cpu.accumulator, 0);
        assert!(cpu.status.contains(CpuFlags::ZERO));
    }

    #[test]
    fn test_tax_negative() {
        let mut cpu = CPU::new();
        cpu.execute_commands(vec![0x_A9, 0b_1000_0101, 0x_AA]);

        assert_eq!(cpu.register_x, 0b_1000_0101);
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
    }

    #[test]
    fn test_tax_zero() {
        let mut cpu = CPU::new();
        cpu.execute_commands(vec![0x_A9, 0, 0x_AA]);

        assert_eq!(cpu.register_x, 0);
        assert!(cpu.status.contains(CpuFlags::ZERO));
    }
}