use crate::instruction::{Instruction, OpCode};
use crate::memory::Memory;
use crate::{Error, MAX_REGISTER_INDEX, MEMORY_SIZE};

pub struct Core {
    registers: [u32; MAX_REGISTER_INDEX as usize + 1],
    memory: Memory,
}

impl Core {
    pub fn new() -> Self {
        let mut core = Core {
            registers: [0u32; MAX_REGISTER_INDEX as usize + 1],
            memory: Memory::new(MEMORY_SIZE),
        };

        // Arbitrary initial registers value.
        // Registers will eventually be initialized through memory loads.
        for (index, register) in core.registers.iter_mut().enumerate() {
            *register = index as u32 * 0x10;
        }

        core
    }

    pub fn register(&self, index: u8) -> Result<u32, Error> {
        if index > MAX_REGISTER_INDEX {
            return Err(Error::Op0OutOfRange);
        }

        Ok(self.registers[index as usize])
    }

    pub fn dump(&self, preamble: &str) {
        println!("do-core1: {}:", preamble);
        for (index, register) in self.registers.iter().enumerate() {
            println!("\tR{}: {:#x?}", index, *register);
        }
    }

    pub fn decode(&mut self, insn: u32) -> Result<Instruction, Error> {
        Instruction::disassemble(insn)
    }

    pub fn execute(&mut self, insn: Instruction) -> Result<(), Error> {
        let opcode = insn.opcode();

        match opcode {
            OpCode::ADD => self.add(insn)?,
            OpCode::XOR => self.xor(insn)?,
            OpCode::LDW => self.load(insn)?,
            OpCode::STW => self.store(insn)?,
        }

        Ok(())
    }

    fn add(&mut self, insn: Instruction) -> Result<(), Error> {
        let op0 = insn.op0() as usize;
        let op1 = insn.op1() as usize;

        self.registers[op0] =
            self.registers[op0]
                .checked_add(self.registers[op1])
                .ok_or(Error::AdditionOverflow(
                    self.registers[op0],
                    self.registers[op1],
                ))?;

        Ok(())
    }

    fn xor(&mut self, insn: Instruction) -> Result<(), Error> {
        let op0 = insn.op0() as usize;
        let op1 = insn.op1() as usize;

        self.registers[op0] = self.registers[op0] ^ self.registers[op1];

        Ok(())
    }

    fn load(&mut self, insn: Instruction) -> Result<(), Error> {
        let op0 = insn.op0() as usize;
        let op1 = insn.op1() as usize;

        self.registers[op0] = self.memory.load(self.registers[op1])?.into();

        Ok(())
    }

    fn store(&mut self, insn: Instruction) -> Result<(), Error> {
        let op0 = insn.op0() as usize;
        let op1 = insn.op1() as usize;

        self.memory
            .store(self.registers[op1], self.registers[op0] as u8)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Core;
    use crate::Error;

    #[test]
    fn test_core_add_r4_r5() -> Result<(), Error> {
        let insn = 0x2902;
        let mut cpu = Core::new();

        let r4 = cpu.register(4)?;
        let r5 = cpu.register(5)?;

        let decoded_insn = cpu.decode(insn)?;
        cpu.execute(decoded_insn)?;

        let new_r4 = cpu.register(4)?;

        assert_eq!(new_r4, r4 + r5);

        Ok(())
    }

    #[test]
    fn test_core_xor_r1_r7() -> Result<(), Error> {
        let insn = 0x3843;
        let mut cpu = Core::new();

        let r1 = cpu.register(1)?;
        let r7 = cpu.register(7)?;

        let decoded_insn = cpu.decode(insn)?;
        cpu.execute(decoded_insn)?;

        let new_r1 = cpu.register(1)?;

        assert_eq!(new_r1, r1 ^ r7);

        Ok(())
    }
}
