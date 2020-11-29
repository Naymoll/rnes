use crate::nes::instruction::Instruction;
use crate::nes::interrupt::{Interrupt, InterruptType, BRK_INT};
use crate::nes::mem::{Memory, Stack};

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
    Stack,
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

//TODO: Сделать название получше
const STACK_ADDRESS: u16 = 0x0100;
const STACK_RESET: u16 = 0x01FF;

const CARRY_MASK: u16 = 256;
const OVERFLOW_MASK: u16 = 128;

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

    //TODO: Проверить
    pub fn execute_commands(&mut self, commands: std::vec::Vec<u8>) {
        loop {
            let opcode = commands[self.program_counter as usize];
            let instruction = Instruction::from_code(opcode);
            self.inc_program_counter(1);

            match opcode {
                //ADC
                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);
                    let value = self.read_u8(address);

                    self.adc(value);
                }
                //AND
                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);
                    let value = self.read_u8(address);

                    self.set_register(Register::Accumulator, self.accumulator & value);
                }
                //AHX
                0x93 | 0x9F => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);

                    self.write_u8(
                        address,
                        self.accumulator & self.register_x & (address >> 8) as u8,
                    );
                }
                //TODO: Проверить
                //ALR
                0x4B => {
                    let value = self.read_u8(self.program_counter);
                    self.set_register(Register::Accumulator, self.accumulator & value);
                    self.lsr_accum();
                }
                //ANC
                0x0B | 0x2B => {
                    let value = self.read_u8(self.program_counter);
                    self.set_register(Register::Accumulator, self.accumulator & value);
                    self.set_carry_flag(self.status.contains(CpuFlags::NEGATIVE));
                }
                //TODO: Проверить правильность установки флагов
                //ARR
                0x6B => {
                    let value = self.read_u8(self.program_counter);
                    self.set_register(Register::Accumulator, self.accumulator & value);
                    self.ror_accum();

                    let bit_5 = (self.accumulator >> 5) & 1;
                    let bit_6 = (self.accumulator >> 6) & 1;

                    self.set_carry_flag(bit_6 == 1);
                    self.set_overflow_flag(bit_5 ^ bit_6 == 1);
                }
                //ASL
                0x0A | 0x06 | 0x16 | 0x0E | 0x1E => {
                    let addressing_mode = instruction.addressing_mode;

                    match addressing_mode {
                        AddressingMode::Accumulator => {
                            self.asl_accum();
                        }
                        _ => {
                            self.asl_mem(addressing_mode);
                        }
                    }
                }
                //TODO: Стоит ли приводить к u16?
                //AXS
                0xCB => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);
                    let value = self.read_u8(address);

                    let and = self.accumulator & self.register_x;
                    let result = (and as u16).wrapping_sub(value as u16);

                    self.set_carry_flag(result & CARRY_MASK != 0);
                    self.set_register(Register::X, result as u8);
                }
                //TODO: Стоит ли делать два раза приведение?
                //BCC
                0x90 => match self.status.contains(CpuFlags::CARRY) {
                    true => {}
                    false => self.branch(),
                },
                //BCS
                0xB0 => match self.status.contains(CpuFlags::CARRY) {
                    true => self.branch(),
                    false => {}
                },
                //BEQ
                0xF0 => match self.status.contains(CpuFlags::ZERO) {
                    true => self.branch(),
                    false => {}
                },
                //BIT
                0x24 | 0x2C => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);
                    let value = self.read_u8(address);

                    self.bit(value);
                }
                //BMI
                0x30 => match self.status.contains(CpuFlags::NEGATIVE) {
                    true => self.branch(),
                    false => {}
                },
                //BNE
                0xD0 => match self.status.contains(CpuFlags::ZERO) {
                    true => {}
                    false => self.branch(),
                },
                //BPL
                0x10 => match self.status.contains(CpuFlags::NEGATIVE) {
                    true => {}
                    false => self.branch(),
                },
                //BRK
                0x00 => {
                    self.interrupt(BRK_INT);
                    self.set_break_command_flag(true);
                }
                //BVC
                0x50 => match self.status.contains(CpuFlags::OVERFLOW) {
                    true => {}
                    false => self.branch(),
                },
                //BVS
                0x70 => match self.status.contains(CpuFlags::OVERFLOW) {
                    true => self.branch(),
                    false => {}
                },
                //CLC
                0x18 => {
                    self.set_carry_flag(false);
                }
                //CLD
                0xD8 => {
                    self.set_decimal_mode_flag(false);
                }
                //CLI
                0x58 => {
                    self.set_interrupt_disable_flag(false);
                }
                //CLV
                0xB8 => {
                    self.set_overflow_flag(false);
                }
                //CMP
                0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);
                    let value = self.read_u8(address);

                    self.compare(self.accumulator, value);
                }
                //CPX
                0xE0 | 0xE4 | 0xEC => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);
                    let value = self.read_u8(address);

                    self.compare(self.register_x, value);
                }
                //CPY
                0xC0 | 0xC4 | 0xCC => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);
                    let value = self.read_u8(address);

                    self.compare(self.register_y, value);
                }
                //TODO: Провярется (value - 1) или value?
                //DCP
                0xC7 | 0xD7 | 0xCF | 0xDF | 0xDB | 0xD3 | 0xC3 => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);
                    let value = self.read_u8(address).wrapping_sub(1); //DEC

                    self.write_u8(address, value);
                    self.compare(self.accumulator, value); //CMP
                }
                //DEC
                0xC6 | 0xD6 | 0xCE | 0xDE => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);
                    let mut value = self.read_u8(address);

                    value = self.decrement(value);
                    self.write_u8(address, value);
                }
                //DEX
                0xCA => {
                    self.register_x = self.decrement(self.register_x);
                }
                //DEY
                0x88 => {
                    self.register_y = self.decrement(self.register_y);
                }
                //EOR
                0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);
                    let value = self.read_u8(address);

                    self.set_register(Register::Accumulator, self.accumulator ^ value);
                }
                //INC
                0xE6 | 0xF6 | 0xEE | 0xFE => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);
                    let mut value = self.read_u8(address);

                    value = self.increment(value);
                    self.write_u8(address, value);
                }
                //INX
                0xE8 => {
                    self.register_x = self.increment(self.register_x);
                }
                //INY
                0xC8 => {
                    self.register_y = self.increment(self.register_y);
                }
                //ISC
                0xE7 | 0xF7 | 0xEF | 0xFF | 0xFB | 0xE3 | 0xF3 => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);
                    let mut value = self.read_u8(address).wrapping_add(1);

                    self.write_u8(address, value);
                    self.sbc(value);
                }
                //JMP
                0x4C | 0x6C => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);

                    self.program_counter = address;
                }
                //JSR
                0x20 => {
                    self.push_u16(self.program_counter.wrapping_add(2));

                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);

                    self.program_counter = address;
                    continue; //Пропускаем увеличение счетчика в конце блока match
                }
                //TODO: Нет четкого описания что делает данная команда.
                // Вместо бесконечного цикла можно использовать NOP,
                // заканчивать выполнение или возращать проц в дефолт
                //KIL
                0x02 | 0x12 | 0x22 | 0x32 | 0x42 | 0x52 | 0x62 | 0x72 | 0x92 | 0xB2 | 0xD2
                | 0xF2 => {
                    //Unofficial opcode, infinite loop
                    loop {}
                }
                //LAS
                0xBB => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);
                    let value = self.read_u8(address) & self.stack_pointer;

                    self.accumulator = value;
                    self.register_x = value;
                    self.stack_pointer = value;

                    self.update_negative_flag(value);
                    self.update_zero_flag(value);
                }
                //LAX
                0xA7 | 0xB7 | 0xAF | 0xBF | 0xA3 | 0xB3 | 0xAB => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);
                    let value = self.read_u8(address);

                    self.accumulator = value;
                    self.register_x = value;

                    self.update_negative_flag(value);
                    self.update_zero_flag(value);
                }
                //LDA
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);
                    let value = self.read_u8(address);

                    self.set_register(Register::Accumulator, value);
                }
                //LDX
                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);
                    let value = self.read_u8(address);

                    self.set_register(Register::X, value);
                }
                //LDY
                0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);
                    let value = self.read_u8(address);

                    self.set_register(Register::Y, value);
                }
                //LSR
                0x4A | 0x46 | 0x56 | 0x4E | 0x5E => {
                    let addressing_mode = instruction.addressing_mode;

                    match addressing_mode {
                        AddressingMode::Accumulator => {
                            self.lsr_accum();
                        }
                        _ => {
                            self.lsr_mem(addressing_mode);
                        }
                    }
                }
                //NOP
                0xEA | 0x80 | 0x82 | 0x89 | 0xC2 | 0xE2 | 0x04 | 0x44 | 0x64 | 0x14 | 0x34
                | 0x54 | 0x74 | 0xD4 | 0xF4 | 0x0C | 0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC
                | 0x1A | 0x3A | 0x5A | 0x7A | 0xDA | 0xFA => {}
                //ORA
                0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);
                    let value = self.read_u8(address);

                    self.set_register(Register::Accumulator, self.accumulator | value);
                }
                //PHA
                0x48 => {
                    self.push_u8(self.accumulator);
                }
                //PHP
                0x08 => {
                    self.push_u8(self.status.bits);
                }
                //PLA
                0x68 => {
                    let stack_val = self.pop_u8();
                    self.set_register(Register::Accumulator, stack_val);
                }
                //PLP
                0x28 => {
                    self.status.bits = self.pop_u8();
                    self.status.insert(CpuFlags::ONE);
                }
                //RLA
                0x27 | 0x37 | 0x2F | 0x3F | 0x3B | 0x33 | 0x23 => {
                    let addressing_mode = instruction.addressing_mode;
                    let value = self.rol_mem(addressing_mode);

                    self.set_register(Register::Accumulator, self.accumulator & value);
                }
                //ROL
                0x2A | 0x26 | 0x36 | 0x2E | 0x3E => {
                    let addressing_mode = instruction.addressing_mode;

                    match addressing_mode {
                        AddressingMode::Accumulator => {
                            self.rol_accum();
                        }
                        _ => {
                            self.rol_mem(addressing_mode);
                        }
                    }
                }
                //ROR
                0x6A | 0x66 | 0x76 | 0x6E | 0x7E => {
                    let addressing_mode = instruction.addressing_mode;

                    match addressing_mode {
                        AddressingMode::Accumulator => {
                            self.ror_accum();
                        }
                        _ => {
                            self.ror_mem(addressing_mode);
                        }
                    }
                }
                //RRA
                0x67 | 0x77 | 0x6F | 0x7F | 0x7B | 0x63 | 0x73 => {
                    let addressing_mode = instruction.addressing_mode;
                    let value = self.ror_mem(addressing_mode);

                    self.adc(value);
                }
                //RTI
                0x40 => {
                    self.status.bits = self.pop_u8();
                    self.program_counter = self.pop_u16();
                }
                //RTS
                0x60 => {
                    self.program_counter = self.pop_u16();
                    continue; //Пропускаем увеличение счетчика в конце блока match
                }
                //SAX
                0x87 | 0x97 | 0x8F | 0x83 => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);

                    self.write_u8(address, self.accumulator & self.register_x);
                }
                //SBC
                0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 | 0xEB => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);
                    let value = self.read_u8(address);

                    self.sbc(value);
                }
                //SEC
                0x38 => {
                    self.set_carry_flag(true);
                }
                //SED
                0xF8 => {
                    self.set_decimal_mode_flag(true);
                }
                //SEI
                0x78 => {
                    self.set_interrupt_disable_flag(true);
                }
                //SHX
                0x9E => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);

                    self.write_u8(address, self.register_x & address.to_be_bytes()[0]);
                }
                //SHY
                0x9C => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);

                    self.write_u8(address, self.register_y & address.to_be_bytes()[0]);
                }
                //SLO
                0x07 | 0x17 | 0x0F | 0x1F | 0x1B | 0x03 | 0x13 => {
                    let addressing_mode = instruction.addressing_mode;
                    let value = self.asl_mem(addressing_mode);

                    self.set_register(Register::Accumulator, self.accumulator | value);
                }
                //SRE
                0x47 | 0x57 | 0x4F | 0x5F | 0x5B | 0x43 | 0x53 => {
                    let addressing_mode = instruction.addressing_mode;
                    let value = self.lsr_mem(addressing_mode);

                    self.set_register(Register::Accumulator, self.accumulator ^ value);
                }
                //STA
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);

                    self.write_u8(address, self.accumulator);
                }
                //STX
                0x86 | 0x96 | 0x8E => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);

                    self.write_u8(address, self.register_x);
                }
                //STY
                0x84 | 0x94 | 0x8C => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);

                    self.write_u8(address, self.register_y);
                }
                //TAS
                0x9B => {
                    self.stack_pointer = self.accumulator & self.register_x;

                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);

                    self.write_u8(address, self.register_y & address.to_be_bytes()[0]);
                }
                //TAX
                0xAA => {
                    self.set_register(Register::X, self.accumulator);
                }
                //TAY
                0xA8 => {
                    self.set_register(Register::Y, self.accumulator);
                }
                //TSX
                0xBA => {
                    self.set_register(Register::X, self.stack_pointer);
                }
                //TXA
                0x8A => {
                    self.set_register(Register::Accumulator, self.register_x);
                }
                //TXS
                0x9A => {
                    self.set_register(Register::Stack, self.register_x);
                }
                //TYA
                0x98 => {
                    self.set_register(Register::Accumulator, self.register_y);
                }
                //XAA
                0x8B => {
                    let addressing_mode = instruction.addressing_mode;
                    let address = self.address(addressing_mode);
                    let value = self.read_u8(address);

                    self.set_register(Register::Accumulator, self.register_x & value);
                }
                _ => unimplemented!("That opcode unimplemented"),
            }

            self.inc_program_counter(instruction.len as u16 - 1);
        }
    }

    //TODO: Подумать над введением 2 аргумента,
    // который можно будет использвоать вместо PC
    // Написать тесты, скорее всего часть сделано неправильно
    fn address(&self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate | AddressingMode::Relative => self.program_counter,

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

            AddressingMode::Implied | AddressingMode::Accumulator => unreachable!(),
        }
    }

    fn interrupt(&mut self, interrupt: Interrupt) {
        let interrupt_disable = self.status.contains(CpuFlags::INTERRUPT_DISABLE);

        if interrupt_disable
            && ((interrupt.int_type == InterruptType::IRQ)
                || (interrupt.int_type == InterruptType::BRK))
        {
            return;
        }

        match interrupt.int_type {
            InterruptType::Reset => {}
            _ => {
                self.push_u16(self.program_counter);
                self.push_u8(self.status.bits);
            }
        }

        self.set_interrupt_disable_flag(true);
        self.program_counter = self.read_u16(interrupt.vec_addr);
    }

    //TODO: Заменить инкремент счеткчика данной функцией,
    // в случае, если не нужно переполнение чисел - заменить на saturation_add()
    fn inc_program_counter(&mut self, value: u16) {
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
        self.status
            .set(CpuFlags::NEGATIVE, (value & 0b1000_0000) != 0);
    }

    fn set_register(&mut self, register: Register, value: u8) {
        match register {
            Register::Accumulator => self.accumulator = value,
            Register::Stack => self.stack_pointer = value,
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

        self.set_register(Register::Accumulator, sum as u8);
    }

    fn asl_accum(&mut self) {
        self.set_carry_flag((self.accumulator & 0b1000_0000) != 0);
        self.set_register(Register::Accumulator, self.accumulator << 1);
    }

    fn asl_mem(&mut self, addressing_mode: AddressingMode) -> u8 {
        let address = self.address(addressing_mode);
        let mut value = self.read_u8(address);

        self.set_carry_flag((value & 0b1000_0000) != 0);

        value <<= 1;
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        self.write_u8(address, value);

        value
    }

    fn bit(&mut self, value: u8) {
        self.update_zero_flag(self.accumulator & value);
        self.set_overflow_flag((value & 0b0100_0000) != 0);
        self.update_negative_flag(value);
    }

    fn branch(&mut self) {
        let offset = self.read_u8(self.program_counter) as i8;
        self.inc_program_counter(offset as u16);
    }

    fn compare(&mut self, lhs: u8, rhs: u8) {
        self.set_carry_flag(lhs >= rhs);

        let result = lhs.wrapping_sub(rhs);
        self.update_zero_flag(result);
        self.update_negative_flag(result);
    }

    fn decrement(&mut self, mut value: u8) -> u8 {
        value = value.wrapping_sub(1);
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        value
    }

    fn increment(&mut self, mut value: u8) -> u8 {
        value = value.wrapping_add(1);
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        value
    }

    fn lsr_accum(&mut self) {
        self.set_carry_flag((self.accumulator & 0b0000_0001) != 0);
        self.set_register(Register::Accumulator, self.accumulator >> 1);
    }

    fn lsr_mem(&mut self, addressing_mode: AddressingMode) -> u8 {
        let address = self.address(addressing_mode);
        let mut value = self.read_u8(address);

        self.set_carry_flag((value & 0b0000_0001) != 0);

        value >>= 1;
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        self.write_u8(address, value);

        value
    }

    fn rol_accum(&mut self) {
        let new_carry = (self.accumulator & 0b1000_0000) != 0;
        let value = self.accumulator << 1
            | match self.status.contains(CpuFlags::CARRY) {
                true => 0b0000_0001,
                false => 0b0000_0000,
            };

        self.set_carry_flag(new_carry);
        self.set_register(Register::Accumulator, value);
    }

    fn rol_mem(&mut self, addressing_mode: AddressingMode) -> u8 {
        let address = self.address(addressing_mode);
        let mut value = self.read_u8(address);

        let new_carry = (value & 0b1000_0000) != 0;
        value <<= 1 | match self.status.contains(CpuFlags::CARRY) {
            true => 0b0000_0001,
            false => 0b0000_0000,
        };

        self.set_carry_flag(new_carry);
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        self.write_u8(address, value);

        value
    }

    fn ror_accum(&mut self) {
        let new_carry = (self.accumulator & 0b0000_0001) != 0;
        let value = self.accumulator >> 1
            | match self.status.contains(CpuFlags::CARRY) {
                true => 0b1000_0000,
                false => 0b0000_0000,
            };

        self.set_carry_flag(new_carry);
        self.set_register(Register::Accumulator, value);
    }

    fn ror_mem(&mut self, addressing_mode: AddressingMode) -> u8 {
        let address = self.address(addressing_mode);
        let mut value = self.read_u8(address);

        let new_carry = (value & 0b0000_0001) != 0;
        value >>= 1 | match self.status.contains(CpuFlags::CARRY) {
            true => 0b1000_0000,
            false => 0b0000_0000,
        };

        self.set_carry_flag(new_carry);
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        self.write_u8(address, value);

        value
    }

    fn sbc(&mut self, value: u8) {
        let carry = match self.status.contains(CpuFlags::CARRY) {
            true => 0,
            false => 1,
        };

        let div = (self.accumulator as u16).wrapping_sub(value as u16 + carry);
        self.set_carry_flag(div & CARRY_MASK != 0);
        self.set_overflow_flag(div & OVERFLOW_MASK != 0);

        self.set_register(Register::Accumulator, div as u8);
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

impl Stack for CPU {
    fn pop_u8(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.read_u8(STACK_ADDRESS + self.stack_pointer as u16)
    }

    fn pop_u16(&mut self) -> u16 {
        self.stack_pointer = self.stack_pointer.wrapping_add(2);
        self.read_u16(STACK_ADDRESS + self.stack_pointer as u16)
    }

    fn push_u8(&mut self, value: u8) {
        self.write_u8(STACK_ADDRESS + self.stack_pointer as u16, value);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn push_u16(&mut self, value: u16) {
        self.write_u16(STACK_ADDRESS + self.stack_pointer as u16, value);
        self.stack_pointer = self.stack_pointer.wrapping_sub(2);
    }
}

//TODO: Поменять тесты
#[cfg(test)]
mod cpu_test {
    use super::*;

    #[test]
    fn test_lda_negative() {
        let mut cpu = CPU::new();
        //cpu.lda(0b1000_0101);

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
        //cpu.lda(0b1000_0101);
        //cpu.tax();

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
