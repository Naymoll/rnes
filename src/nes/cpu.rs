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

enum Register {
    Accumulator,
    X,
    Y,
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

const CARRY_MASK: u16 = 256;
const OVERFLOW_MASK: u16 = 128;

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
                let address = self.read_u16(self.program_counter);
                self.read_u16(address.wrapping_add(self.register_x as u16))
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

    //TODO: Заменить инкремент счеткчика данной функцией,
    // в случае, если не нужно переполнение чисел - заменить на saturation_add()
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

    fn set_register(&mut self, register: Register, value: u8) {
        match register {
            Register::Accumulator => self.accumulator = value,
            Register::X => self.register_x = value,
            Register::Y => self.register_y = value,
        }

        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn adc(&mut self, value: u8) {
        let carry = match self.status.contains(CpuFlags::CARRY) {
            true => 1,
            false => 0,
        };

        let sum = self.accumulator as u16 + value as u16 + carry;
        self.set_carry_flag(sum & CARRY_MASK != 0);
        self.set_overflow_flag(sum & OVERFLOW_MASK != 0);

        self.set_register(Register::Accumulator,sum as u8);
    }

    fn and(&mut self, value: u8) {
        self.set_register(Register::Accumulator, self.accumulator & value);
    }

    //TODO: Исправить (должно быть присваивание) и протестировать
    fn asl(&mut self, mut value: u8) -> u8 {
        self.set_carry_flag((value & 0b1000_0000) != 0);

        value <<= 1;
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        value
    }

    fn bit(&mut self, value: u8) {
        self.update_zero_flag(self.accumulator & value);
        self.set_overflow_flag((value & 0b0100_000) != 0);
        self.update_negative_flag(value);
    }

    //TODO: Стоит убрать, а очистку/установку делать напрямую
    fn clc(&mut self) {
        self.set_carry_flag(false);
    }

    fn cld(&mut self) {
        self.set_decimal_mode_flag(false);
    }

    fn cli(&mut self) {
        self.set_interrupt_disable_flag(false);
    }

    fn clv(&mut self) {
        self.set_overflow_flag(false);
    }

    fn compare(&mut self, lhs: u8, rhs: u8) {
        self.set_carry_flag(lhs >= rhs);

        let result = lhs.wrapping_sub(rhs);
        self.update_zero_flag(result);
        self.update_negative_flag(result);
    }

    fn cmp(&mut self, value: u8) {
        self.compare(self.accumulator, value);
    }

    fn cpx(&mut self, value: u8) {
        self.compare(self.register_x, value);
    }

    fn cpy(&mut self, value: u8) {
        self.compare(self.register_y, value);
    }

    fn decrement(&mut self, mut value: u8) -> u8 {
        value = value.wrapping_sub(1);
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        value
    }

    //TODO: Возможо, стоит поменять сигнатуру.
    // Либо попробовать другой метод с передачей режима адресации как аргумент
    fn dec(&mut self, address: u16) {
        let mut value = self.read_u8(address);
        value = self.decrement(value);
        self.write_u8(address, value);
    }

    fn dex(&mut self) {
        self.register_x = self.decrement(self.register_x);
    }

    fn dey(&mut self) {
        self.register_y = self.decrement(self.register_y);
    }

    fn eor(&mut self, value: u8) {
        self.set_register(Register::Accumulator, self.accumulator ^ value);
    }

    fn increment(&mut self, mut value: u8) -> u8 {
        value = value.wrapping_add(1);
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        value
    }

    fn inc(&mut self, address: u16) {
        let mut value = self.read_u8(address);
        value = self.increment(value);
        self.write_u8(address, value);
    }

    fn inx(&mut self) {
        self.register_x = self.increment(self.register_x);
    }

    fn iny(&mut self) {
        self.register_y = self.increment(self.register_y);
    }

    fn jmp(&mut self, address: u16) {
        self.program_counter = address;
    }

    fn lda(&mut self, value: u8) {
        self.set_register(Register::Accumulator, value);
    }

    fn ldx(&mut self, value: u8) {
        self.set_register(Register::X, value);
    }

    fn ldy(&mut self, value: u8) {
        self.set_register(Register::Y, value);
    }

    //TODO: Исправить (должно быть присваивание) и протестировать
    fn lsr(&mut self, mut value: u8) -> u8 {
        self.set_carry_flag((value & 0b0000_0001) != 0);

        value >>= 1;
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        value
    }

    fn nop(&mut self) {
        self.increment_program_counter(1);
    }

    fn ora(&mut self, value: u8) {
        self.set_register(Register::Accumulator, self.accumulator | value);
    }

    fn rol(&mut self, mut value: u8) -> u8 {
        let new_carry = (value & 0b1000_0000) != 0;

        value = (value << 1) | match self.status.contains(CpuFlags::CARRY) {
            true => 0b0000_0001,
            false => 0b0000_0000,
        };

        self.set_carry_flag(new_carry);
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        value
    }

    fn ror(&mut self, mut value: u8) -> u8 {
        let new_carry = (value & 0b0000_0001) != 0;

        value = (value >> 1) | match self.status.contains(CpuFlags::CARRY) {
            true => 0b1000_0000,
            false => 0b0000_0000,
        };

        self.set_carry_flag(new_carry);
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        value
    }

    fn sbc(&mut self, value: u8) {
        let carry = match self.status.contains(CpuFlags::CARRY) {
            true => 0,
            false => 1,
        };

        let div = self.accumulator as u16 - value as u16 - carry;
        self.set_carry_flag(div & CARRY_MASK != 0);
        self.set_overflow_flag(div & OVERFLOW_MASK != 0);

        self.set_register(Register::Accumulator, div as u8);
    }

    fn store(&mut self, address: u16, value: u8) {
        self.write_u8(address, value);
    }

    fn sta(&mut self, address: u16) {
        self.store(address, self.accumulator);
    }

    fn stx(&mut self, address: u16) {
        self.store(address, self.register_x);
    }

    fn sty(&mut self, address: u16) {
        self.store(address, self.register_y);
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

    fn write_u16(&mut self, address: u16, value: u16) {
        let bytes = value.to_le_bytes();
        self.memory[address as usize] = bytes[0];
        self.memory[address.wrapping_add(1) as usize] = bytes[1];
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