#[macro_export]
macro_rules! instruction {
    ($struct_name:ident, $name:expr, $opcode:expr, $len:expr, $cycle:expr, $op:expr) => {
        pub struct $struct_name;

        impl crate::instructions::Instruction for $struct_name {
            fn execute(&mut self, ctx: &mut crate::instructions::Context) {
                $op(self, ctx)
            }

            #[inline(always)]
            fn name(&self) -> &str { $name }
            #[inline(always)]
            fn opcode(&self) -> u8 { $opcode }
            #[inline(always)]
            fn len(&self) -> u8 { $len }
            #[inline(always)]
            fn cycle(&self) -> u8 { $cycle }
        }

        impl TryFrom<u8> for $struct_name {
            type Error = crate::instructions::InvalidOpcodeError;

            fn try_from(opcode: u8) -> Result<Self, Self::Error> {
                if opcode == $opcode {
                    Ok($struct_name)
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
        instruction!(TestInstruction, "Test", 67, 7, 5, {});
    }
}
