//! This crate permits to simulate a couple of processor instructions.

use clap::Parser;

#[allow(dead_code)]
#[derive(Debug)]
struct Instruction {
    opcode: OpCode,
    op0: u8,
    op1: u8,
}

impl Instruction {
    // Instruction constructor, a.k.a. disassembler.
    fn disassemble(insn: u32) -> Instruction {
        // Keep the first 6 bits only
        let opcode = OpCode::from_u8((insn & 0x3f) as u8);

        // Shift right by 6, keep only the first 5 bits.
        let op0 = ((insn >> 6) & 0x1f) as u8;

        // Shift right by 11, keep only the first 5 bits.
        let op1: u8 = ((insn >> 11) & 0x1f) as u8;

        Instruction { opcode, op0, op1 }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum OpCode {
    LDW = 0x00,
    STW = 0x01,
    ADD = 0x02,
    XOR = 0x03,
    SHR = 0x04,
    SHL = 0x05,
}

impl OpCode {
    fn from_u8(opcode: u8) -> OpCode {
        match opcode {
            0x00 => OpCode::LDW,
            0x01 => OpCode::STW,
            0x02 => OpCode::ADD,
            0x03 => OpCode::XOR,
            0x04 => OpCode::SHR,
            0x05 => OpCode::SHL,
            _ => panic!("Unknown opcode {:?}", opcode),
        }
    }
}

/// Returns the sum of the two arguments
fn add(op0: u32, op1: u32) -> u32 {
    op0 + op1
}

/// Returns the xor result of the two arguments
fn xor(op0: u32, op1: u32) -> u32 {
    op0 ^ op1
}

/// Returns the right shifting op1 bits to op0
fn shr(op0: u32, op1: u32) -> u32 {
    op0 >> op1
}

/// Returns the left shifting op1 bits to op0
fn shl(op0: u32, op1: u32) -> u32 {
    op0 << op1
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    instruction: u32,
}

/// Simple program to run a processor instruction
fn main() {
    let args = Args::parse();

    // ADD R1, R3 -> Opcode is 2 (ADD), op0 is 1 (R1) and op1 is 3 (R3)
    // The first 6 bits of the instruction are the opcode (2): 0b000010
    // Bits 6 to 10 are for op0 (1): 0b000001
    // Bits 11 to 15 are for op1 (3): 0b000011
    // The instruction for ADD R1, R3 is: 00011 | 00001 | 000010, i.e. 0b0001100001000010
    //
    // When splitting this binary representation in groups of 4 bits, this looks like:
    // 0001 1000 0100 0010
    //  1     8   4    2
    // 0b0001100001000010 = 0x1842
    // let insn: u32 = 0x1845;
    let insn: u32 = args.instruction;
    let mut r1: u32 = 5;
    let r3: u32 = 2;

    println!(
        "do-core-1: instruction {:#x?} Initial CPU state [R1:{:#x?} R3:{:#x?}]",
        insn, r1, r3
    );

    let decoded_instruction = Instruction::disassemble(insn);
    println!(
        "do-core-1: instruction decoded into {:?}",
        decoded_instruction
    );

    match decoded_instruction.opcode {
        OpCode::ADD => r1 = add(r1, r3),
        OpCode::XOR => r1 = xor(r1, r3),
        OpCode::SHR => r1 = shr(r1, r3),
        OpCode::SHL => r1 = shl(r1, r3),
        _ => panic!("Unknown opcode {:?}", decoded_instruction.opcode),
    }

    println!(
        "do-core-1: instruction {:#x?} Final CPU state [R1:{:#x?} R3:{:#x?}]",
        insn, r1, r3
    );
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, OpCode};
    
    #[test]
    fn test_instruction_disassemble_ldw_r1_r3() {
        let insn_bytes: u32 = 0x1840;
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::LDW);
        assert_eq!(insn.op0, 1);
        assert_eq!(insn.op1, 3);
    }
    
    #[test]
    fn test_instruction_disassemble_stw_r1_r3() {
        let insn_bytes: u32 = 0x1841;
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::STW);
        assert_eq!(insn.op0, 1);
        assert_eq!(insn.op1, 3);
    }
    
    #[test]
    fn test_instruction_disassemble_add_r1_r3() {
        let insn_bytes: u32 = 0x1842;
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::ADD);
        assert_eq!(insn.op0, 1);
        assert_eq!(insn.op1, 3);
    }
    
    #[test]
    fn test_instruction_compute_add() {
        assert_eq!(8, crate::add(5, 3));
    }
    
    #[test]
    fn test_instruction_disassemble_xor_r1_r3() {
        let insn_bytes: u32 = 0x1843;
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::XOR);
        assert_eq!(insn.op0, 1);
        assert_eq!(insn.op1, 3);
    }
    
    #[test]
    fn test_instruction_compute_xor() {
        assert_eq!(6, crate::xor(5, 3));
    }
    
    #[test]
    fn test_instruction_disassemble_shr_r1_r3() {
        let insn_bytes: u32 = 0x1844;
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::SHR);
        assert_eq!(insn.op0, 1);
        assert_eq!(insn.op1, 3);
    }
    
    #[test]
    fn test_instruction_compute_shr_r1_r3() {
        assert_eq!(2, crate::shr(5, 1));
    }
    
    #[test]
    fn test_instruction_disassemble_shl_r1_r3() {
        let insn_bytes: u32 = 0x1845;
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::SHL);
        assert_eq!(insn.op0, 1);
        assert_eq!(insn.op1, 3);
    }
    
    #[test]
    fn test_instruction_compute_shl_r1_r3() {
        assert_eq!(10, crate::shl(5, 1));
    }
}
