use clap::Parser;

#[derive(Parser)]
#[clap(version, author)]
struct DoCoreOpts {
    /// DO Core instruction
    #[clap(short, long)]
    insn: String,
}

#[derive(Debug)]
struct Instruction {
    opcode: OpCode,
    op0: u8,
    op1: u8,
}

#[derive(Debug)]
enum Error {
    Op0OutOfRange,
    Op1OutOfRange,
}

// do-core1 register indexes range from 0 to 7.
const MAX_REGISTER_INDEX: u8 = 7;

impl Instruction {
    // Instruction constructor, a.k.a. disassembler.
    fn disassemble(insn: u32) -> Result<Instruction, Error> {
        // Keep the first 6 bits only
        let opcode = OpCode::from_u8((insn & 0x3f) as u8);

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
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum OpCode {
    LDW = 0x00,
    STW = 0x01,
    ADD = 0x02,
    XOR = 0x03,
}

impl OpCode {
    fn from_u8(opcode: u8) -> OpCode {
        match opcode {
            0x00 => OpCode::LDW,
            0x01 => OpCode::STW,
            0x02 => OpCode::ADD,
            0x03 => OpCode::XOR,
            _ => panic!("Unknown opcode {:?}", opcode),
        }
    }
}
fn add(op0: u32, op1: u32) -> u32 {
    op0 + op1
}

fn xor(op0: u32, op1: u32) -> u32 {
    op0 ^ op1
}

fn main() -> Result<(), Error> {
    let opts: DoCoreOpts = DoCoreOpts::parse();
    let insn = u32::from_str_radix(opts.insn.trim_start_matches("0x"), 16).unwrap();
    let mut r1: u32 = 20;
    let r3 = 12;

    println!(
        "do-core-1: instruction {:#x?} Initial CPU state [R1:{:#x?} R3:{:#x?}]",
        insn, r1, r3
    );

    let decoded_instruction = match Instruction::disassemble(insn) {
        Ok(insn) => insn,
        Err(e) => return Err(e),
    };
    println!(
        "do-core-1: instruction decoded into {:?}",
        decoded_instruction
    );

    match decoded_instruction.opcode {
        OpCode::ADD => r1 = add(r1, r3),
        OpCode::XOR => r1 = xor(r1, r3),
        _ => panic!("Unknown opcode {:?}", decoded_instruction.opcode),
    }

    println!(
        "do-core-1: instruction {:#x?} Final CPU state [R1:{:#x?} R3:{:#x?}]",
        insn, r1, r3
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, OpCode};

    #[test]
    fn test_instruction_disassemble_add_r1_r3() {
        let insn_bytes: u32 = 0x1842;
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::ADD);
        assert_eq!(insn.op0, 1);
        assert_eq!(insn.op1, 3);
    }

    #[test]
    fn test_instruction_disassemble_add_r7_r2() {
        let insn_bytes: u32 = 0x11c2;
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::ADD);
        assert_eq!(insn.op0, 7);
        assert_eq!(insn.op1, 2);
    }

    #[test]
    fn test_instruction_disassemble_ldw_r0_r1() {
        let insn_bytes: u32 = 0x0800;
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::LDW);
        assert_eq!(insn.op0, 0);
        assert_eq!(insn.op1, 1);
    }

    #[test]
    fn test_instruction_disassemble_xor_r2_r3() {
        let insn_bytes: u32 = 0x1883;
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::XOR);
        assert_eq!(insn.op0, 2);
        assert_eq!(insn.op1, 3);
    }

    #[test]
    fn test_instruction_disassemble_stw_r5_r0() {
        let insn_bytes: u32 = 0x0141;
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::STW);
        assert_eq!(insn.op0, 5);
        assert_eq!(insn.op1, 0);
    }
}
