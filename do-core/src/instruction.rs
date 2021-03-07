use crate::{Error, MAX_REGISTER_INDEX};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum OpCode {
    LDW = 0x00,
    STW = 0x01,
    ADD = 0x02,
    XOR = 0x03,
}

impl OpCode {
    pub fn from_u8(opcode: u8) -> Result<OpCode, Error> {
        match opcode {
            0x00 => Ok(OpCode::LDW),
            0x01 => Ok(OpCode::STW),
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
    pub fn disassemble(insn: u32) -> Result<Instruction, Error> {
        // Keep the first 6 bits only
        let opcode = OpCode::from_u8((insn & 0x3f) as u8)?;

        // Shift right by 6, keep only the first 5 bits.
        let op0 = ((insn >> 6) & 0x1f) as u8;

        // Shift right by 11, keep only the first 5 bits.
        let op1: u8 = ((insn >> 11) & 0x1f) as u8;

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
    use crate::instruction::{Instruction, OpCode};
    use crate::Error;

    #[test]
    fn test_instruction_disassemble_add_r1_r3() -> Result<(), Error> {
        let insn_bytes: u32 = 0x1842;
        let insn = Instruction::disassemble(insn_bytes)?;

        assert_eq!(insn.opcode, OpCode::ADD);
        assert_eq!(insn.op0, 1);
        assert_eq!(insn.op1, 3);

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_badop_r9_r1() -> Result<(), Error> {
        // Use all 6 bytes for the opcode.
        // It should be invalid for a while...
        let insn_bytes: u32 = 0x067f;
        assert!(Instruction::disassemble(insn_bytes).is_err());

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_add_r0_r10() -> Result<(), Error> {
        let insn_bytes: u32 = 0x20a;
        assert!(Instruction::disassemble(insn_bytes).is_err());

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_add_r7_r2() -> Result<(), Error> {
        let insn_bytes: u32 = 0x11c2;
        let insn = Instruction::disassemble(insn_bytes)?;

        assert_eq!(insn.opcode, OpCode::ADD);
        assert_eq!(insn.op0, 7);
        assert_eq!(insn.op1, 2);

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_ldw_r0_r1() -> Result<(), Error> {
        let insn_bytes: u32 = 0x0800;
        let insn = Instruction::disassemble(insn_bytes)?;

        assert_eq!(insn.opcode, OpCode::LDW);
        assert_eq!(insn.op0, 0);
        assert_eq!(insn.op1, 1);

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_xor_r2_r3() -> Result<(), Error> {
        let insn_bytes: u32 = 0x1883;
        let insn = Instruction::disassemble(insn_bytes)?;

        assert_eq!(insn.opcode, OpCode::XOR);
        assert_eq!(insn.op0, 2);
        assert_eq!(insn.op1, 3);

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_stw_r5_r0() -> Result<(), Error> {
        let insn_bytes: u32 = 0x0141;
        let insn = Instruction::disassemble(insn_bytes)?;

        assert_eq!(insn.opcode, OpCode::STW);
        assert_eq!(insn.op0, 5);
        assert_eq!(insn.op1, 0);

        Ok(())
    }
}
