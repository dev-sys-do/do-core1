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

impl std::str::FromStr for OpCode {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LD" => Ok(OpCode::LD),
            "ST" => Ok(OpCode::ST),
            "ADD" => Ok(OpCode::ADD),
            "XOR" => Ok(OpCode::XOR),
            _ => Err(Error::ParseOpError),
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

    pub fn opcode(&self) -> OpCode {
        self.opcode.clone()
    }

    pub fn op0(&self) -> u8 {
        self.op0
    }

    pub fn op1(&self) -> u8 {
        self.op1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    use std::str::FromStr;

    #[test]
    fn test_instruction_disassemble_add_r0_r1() -> Result<(), Error> {
        let insn_bytes: u16 = 0x201;
        let insn = Instruction::disassemble(insn_bytes)?;

        assert_eq!(insn.opcode, OpCode::ADD);
        assert_eq!(insn.op0, 0);
        assert_eq!(insn.op1, 1);

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_add_r9_r1() -> Result<(), Error> {
        let insn_bytes: u16 = 0x291;
        assert!(Instruction::disassemble(insn_bytes).is_err());

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_add_r0_r10() -> Result<(), Error> {
        let insn_bytes: u16 = 0x20a;
        assert!(Instruction::disassemble(insn_bytes).is_err());

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_add_r7_r2() -> Result<(), Error> {
        let insn_bytes: u16 = 0x272;
        let insn = Instruction::disassemble(insn_bytes)?;

        assert_eq!(insn.opcode, OpCode::ADD);
        assert_eq!(insn.op0, 7);
        assert_eq!(insn.op1, 2);

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_ld_r0_r1() -> Result<(), Error> {
        let insn_bytes: u16 = 0x01;
        let insn = Instruction::disassemble(insn_bytes)?;

        assert_eq!(insn.opcode, OpCode::LD);
        assert_eq!(insn.op0, 0);
        assert_eq!(insn.op1, 1);

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_xor_r2_r3() -> Result<(), Error> {
        let insn_bytes: u16 = 0x323;
        let insn = Instruction::disassemble(insn_bytes)?;

        assert_eq!(insn.opcode, OpCode::XOR);
        assert_eq!(insn.op0, 2);
        assert_eq!(insn.op1, 3);

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_st_r5_r0() -> Result<(), Error> {
        let insn_bytes: u16 = 0x150;
        let insn = Instruction::disassemble(insn_bytes)?;

        assert_eq!(insn.opcode, OpCode::ST);
        assert_eq!(insn.op0, 5);
        assert_eq!(insn.op1, 0);

        Ok(())
    }

    #[test]
    fn test_opcode_from_string() -> Result<(), Error> {
        assert_eq!(OpCode::from_str("ADD").unwrap(), OpCode::ADD);
        assert_eq!(OpCode::from_str("LD").unwrap(), OpCode::LD);

        assert!(OpCode::from_str("GIBBERISH").is_err());

        Ok(())
    }
}
