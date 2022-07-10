macro_rules! instruction {
    ($opcode:expr, $len:expr, $cycle:expr, $name:ident, $op:block) => {
        pub struct $name;

        impl crate::instructions::Instruction for $name {
            fn execute(&mut self, _ctx: &mut crate::instructions::Context) {
                $op
            }

            #[inline(always)]
            fn opcode(&self) -> u8 { $opcode }
            #[inline(always)]
            fn len(&self) -> u8 { $len }
            #[inline(always)]
            fn cycle(&self) -> u8 { $cycle }
        }

        impl TryFrom<u8> for $name {
            type Error = crate::instructions::InvalidOpcodeError;

            fn try_from(opcode: u8) -> Result<Self, Self::Error> {
                if opcode == $opcode {
                    Ok($name)
                } else {
                    Err(Self::Error::new($opcode, opcode))
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    fn macro_test() {
        instruction!(67, 7, 5, TestInstruction, {});
    }
}
