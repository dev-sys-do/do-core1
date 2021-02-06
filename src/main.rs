extern crate clap;
use clap::{App, Arg};

#[derive(Debug)]
struct Instruction {
    opcode: OpCode,
    op0: u8,
    op1: u8,
}

#[derive(Debug)]
enum Error {
    InvalidOpCode(u8),
    Op0OutOfRange,
    Op1OutOfRange,
    AdditionOverflow(u16, u16),
}

// do-core1 register indexes range from 0 to 7.
const MAX_REGISTER_INDEX: u8 = 7;

impl Instruction {
    // Instruction constructor, a.k.a. disassembler.
    fn disassemble(insn: u16) -> Result<Instruction, Error> {
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

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum OpCode {
    LD = 0x00,
    ST = 0x01,
    ADD = 0x02,
    XOR = 0x03,
}

impl OpCode {
    fn from_u8(opcode: u8) -> Result<OpCode, Error> {
        match opcode {
            0x00 => Ok(OpCode::LD),
            0x01 => Ok(OpCode::ST),
            0x02 => Ok(OpCode::ADD),
            0x03 => Ok(OpCode::XOR),
            _ => Err(Error::InvalidOpCode(opcode)),
        }
    }
}

fn add(op0: u16, op1: u16) -> Result<u16, Error> {
    op0.checked_add(op1)
        .ok_or(Error::AdditionOverflow(op0, op1))
}

fn xor(op0: u16, op1: u16) -> u16 {
    op0 ^ op1
}

fn main() -> Result<(), Error> {
    let arguments = App::new("do-core1")
        .about("do-core1 emulator")
        .arg(
            Arg::with_name("instruction")
                .long("instruction")
                .help("do-core1 instruction to execute")
                .takes_value(true),
        )
        .get_matches();

    let insn_string = arguments
        .value_of("instruction")
        .expect("Missing --instruction argument")
        .trim_start_matches("0x");

    // Convert an hexadecimal formatted string into a u16
    let insn = u16::from_str_radix(insn_string, 16).unwrap();
    let mut r0: u16 = 10;
    let r1: u16 = 20;

    println!(
        "do-core-1: instruction {:#x?} Initial CPU state [R0:{:#x?} R1:{:#x?}]",
        insn, r0, r1
    );

    let decoded_instruction = Instruction::disassemble(insn)?;
    println!(
        "do-core-1: instruction decoded into {:?}",
        decoded_instruction
    );

    match decoded_instruction.opcode {
        OpCode::ADD => r0 = add(r0, r1)?,
        OpCode::XOR => r0 = xor(r0, r1),
        _ => panic!("Unknown opcode {:?}", decoded_instruction.opcode),
    }

    println!(
        "do-core-1: instruction {:#x?} Final CPU state [R0:{:#x?} R1:{:#x?}]",
        insn, r0, r1
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Error, Instruction, OpCode};

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
}
