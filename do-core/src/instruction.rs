use crate::{Error, MAX_REGISTER_INDEX};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum OpCode {
    LD = 0x00,
    ST = 0x01,
    ADD = 0x02,
    XOR = 0x03,
}

impl OpCode {
    pub fn from_u8(opcode: u8) -> Result<OpCode, Error> {
        match opcode {
            0x00 => Ok(OpCode::LD),
            0x01 => Ok(OpCode::ST),
            0x02 => Ok(OpCode::ADD),
            0x03 => Ok(OpCode::XOR),
            _ => Err(Error::InvalidOpCode(opcode)),
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    opcode: OpCode,
    op0: u8,
    op1: u8,
}

impl Instruction {
    // Instruction constructor, a.k.a. disassembler.
    pub fn disassemble(insn: u16) -> Result<Instruction, Error> {
        let opcode = OpCode::from_u8((insn >> 8) as u8)?;
        let op0 = ((insn & 0xf0) >> 4) as u8;
        let op1: u8 = (insn & 0xf) as u8;

        if op0 > MAX_REGISTER_INDEX {
            return Err(Error::Op0OutOfRange);
        }

        if op1 > MAX_REGISTER_INDEX {
            return Err(Error::Op1OutOfRange);
        }

        Ok(Instruction { opcode, op0, op1 })
    }
}
