use crate::instruction::{Instruction, OpCode};
use crate::{Error, MAX_REGISTER_INDEX};

pub struct Core {
    registers: [u32; MAX_REGISTER_INDEX as usize + 1],
}

impl Core {
    pub fn new() -> Self {
        let mut core = Core {
            registers: [0u32; MAX_REGISTER_INDEX as usize + 1],
        };

        // Arbitrary initial registers value.
        // Registers will eventually be initialized through memory loads.
        for (index, register) in core.registers.iter_mut().enumerate() {
            *register = index as u32 * 0x10;
        }

        core
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
            OpCode::LDW | OpCode::STW => return Err(Error::UnsupportedOpCode(opcode)),
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
}
