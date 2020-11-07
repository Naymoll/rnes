use crate::nes::mem::Memory;

pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
    Accumulator,
    Implied,
}

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
    pub memory: [u8; 65536],
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
            memory: [0; 65536],
        }
    }

    //TODO: Подумать над введением 2 аргумента,
    // который можно будет использвоать вместо PC
    // Написать тесты, скорее всего часть сделано неправильно
    fn address(&self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,

            AddressingMode::ZeroPage => self.read_u8(self.program_counter) as u16,

            AddressingMode::ZeroPageX => {
                let base = self.read_u8(self.program_counter) as u16;
                base.wrapping_add(self.register_x as u16)
            }

            AddressingMode::ZeroPageY => {
                let base = self.read_u8(self.program_counter) as u16;
                base.wrapping_add(self.register_y as u16)
            }

            AddressingMode::Absolute => self.read_u16(self.program_counter),

            AddressingMode::AbsoluteX => {
                let base = self.read_u16(self.program_counter);
                base.wrapping_add(self.register_x as u16)
            }

            AddressingMode::AbsoluteY => {
                let base = self.read_u16(self.program_counter);
                base.wrapping_add(self.register_y as u16)
            }

            AddressingMode::Indirect => {
                let address = self.read_u16(self.program_counter);
                self.read_u16(address)
            }

            AddressingMode::IndirectX => {
                let address = self.read_u16(self.program_counter)
                    .wrapping_add(self.register_x as u16);

                self.read_u16(address)
            }

            AddressingMode::IndirectY => {
                let address = self.read_u16(self.program_counter);
                self.read_u16(address.wrapping_add(self.register_y as u16))
            }

            //TODO: Проверить корректность saturating_abs
            AddressingMode::Relative => {
                let offset = self.read_u8(self.program_counter) as i8;
                match offset.is_negative() {
                    true => self.program_counter.wrapping_sub((offset.saturating_abs() as u16) + 1),
                    false => self.program_counter.wrapping_add(offset as u16)
                }
            }

            AddressingMode::Implied | AddressingMode::Accumulator => unreachable!(),
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
        self.status.set(CpuFlags::NEGATIVE, (value & 0b1000_0000) != 0);
    }

    fn set_accumulator(&mut self, value: u8) {
        self.accumulator = value;

        self.update_zero_flag(self.accumulator);
        self.update_negative_flag(self.accumulator);
    }

    fn add_to_accumulator(&mut self, value: u8) {
        let carry = match self.status.contains(CpuFlags::CARRY) {
            true => 1,
            false => 0,
        };

        let sum = self.accumulator as u16 + value as u16 + carry;

        const CARRY_MASK: u16 = 256;
        const OVERFLOW_MASK: u16 = 128;

        self.status.set(CpuFlags::CARRY, sum & CARRY_MASK != 0);
        self.status.set(CpuFlags::OVERFLOW, sum & OVERFLOW_MASK != 0);

        self.set_accumulator(sum as u8);
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

impl Memory for CPU {
    fn read_u8(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn read_u16(&self, address: u16) -> u16 {
        let lo = self.memory[address as usize];
        let hi = self.memory[address.wrapping_add(1) as usize];

        u16::from_le_bytes([lo, hi])
    }

    fn write_u8(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    //TODO: Проверить порядок
    fn write_u16(&mut self, address: u16, value: u16) {
        let bytes = value.to_be_bytes();
        self.memory[address as usize] = bytes[0];
        self.memory[address as usize] = bytes[1];
    }
}

#[cfg(test)]
mod cpu_test {
    use super::*;

    #[test]
    fn test_lda_negative() {
        let mut cpu = CPU::new();
        cpu.lda(0b1000_0101);

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
        cpu.lda(0b1000_0101);
        cpu.tax();

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

    #[test]
    fn test_adc_overflow() {
        let mut cpu = CPU::new();
        cpu.execute_commands(vec![0xA9, 255, 0x69, 129]);

        assert_eq!(cpu.accumulator, 128);
        assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    }

    #[test]
    fn test_adc_carry() {
        let mut cpu = CPU::new();
        cpu.execute_commands(vec![0xA9, 128, 0x69, 128]);

        assert_eq!(cpu.accumulator, 0);
        assert!(cpu.status.contains(CpuFlags::CARRY));
    }

    #[test]
    fn test_zero_addressing_mode() {
        let mut cpu = CPU::new();
        cpu.program_counter = 2;
        cpu.memory[2] = 0x15;
        cpu.memory[21] = 10;
        let address = cpu.address(AddressingMode::ZeroPage);
        let value = cpu.read_u8(address);

        assert_eq!(value, 10);
    }

    #[test]
    fn test_absolute_addressing_mode() {
        let mut cpu = CPU::new();
        cpu.program_counter = 2;
        cpu.memory[2] = 0x15;
        cpu.memory[3] = 0x10;
        cpu.memory[0x1015] = 10;
        let address = cpu.address(AddressingMode::Absolute);
        let value = cpu.read_u8(address);

        assert_eq!(value, 10);
    }

    #[test]
    fn test_indirect_addressing_mode() {
        let mut cpu = CPU::new();
        cpu.program_counter = 2;
        cpu.memory[2] = 0x15;
        cpu.memory[3] = 0x10;
        cpu.memory[0x1015] = 10;
        cpu.memory[0x1016] = 20;
        let ptr = cpu.address(AddressingMode::Indirect);

        assert_eq!(ptr, 5130);
    }
}