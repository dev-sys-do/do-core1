use clap::Parser;

/// Represent an instruction
/// with an opcode and two operands
#[allow(dead_code)]
#[derive(Debug)]
struct Instruction {
    /// the operation code
    opcode: OpCode,
    /// the first operand
    op0: u8,
    /// the second operand
    op1: u8,
}

/// Simple cli to execute an instruction from the do-core1
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct DoCoreArgs {
    /// Instruction to execute (eg. 0x1842)
    #[clap(short, long)]
    insn: String,
}

impl Instruction {
    /// Instruction constructor, a.k.a. disassembler.
    /// Take an instruction code as u32 and return an Instruction instance
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

/// Represent an operation code from the do-core
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
    /// Convert an u8 to an OpCode, panic if it's not a valid OpCode
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

/// Returns the result of the addition of the two parameters
fn add(op0: u32, op1: u32) -> u32 {
    op0 + op1
}

/// Return the XOR operation of the two parameters
fn xor(op0: u32, op1: u32) -> u32 {
    op0 ^ op1
}

/// Return the right shift operation of the two parameters
fn shr(op0: u32, op1: u8) -> u32 {
    op0 >> op1
}

/// Return the left shift operation of the two parameters
fn shl(op0: u32, op1: u8) -> u32 {
    op0 << op1
}

/// Launch the cli, parse the arguments and execute the instruction
fn main() {
    let args: DoCoreArgs = DoCoreArgs::parse();
    let insn: u32 = u32::from_str_radix(args.insn.trim_start_matches("0x"), 16).unwrap();
    let mut r1: u32 = 20;
    let r3: u32 = 12;

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
        OpCode::SHL => r1 = shr(r1, decoded_instruction.op1),
        OpCode::SHR => r1 = shl(r1, decoded_instruction.op1),
        _ => panic!("Unknown opcode {:?}", decoded_instruction.opcode),
    }

    println!(
        "do-core-1: instruction {:#x?} Final CPU state [R1:{:#x?} R3:{:#x?}]",
        insn, r1, r3
    );
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, OpCode, shl, shr};

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

    #[test]
    fn test_instruction_disassemble_shr_r1_r0() {
        let insn_bytes: u32 = 0x0804;
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::SHR);
        assert_eq!(insn.op0, 0);
        assert_eq!(insn.op1, 1);
    }

    #[test]
    fn test_instruction_disassemble_shl_r6_r2() {
        let insn_bytes: u32 = 0x1185;
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::SHL);
        assert_eq!(insn.op0, 6);
        assert_eq!(insn.op1, 2);
    }

    #[test]
    fn test_function_shl() {
        let op0: u32 = 0x05;
        let op1: u8 = 0x01;
        let result = shl(op0, op1);

        assert_eq!(result, 0x0A);
    }

    #[test]
    fn test_function_shr() {
        let op0: u32 = 0x10;
        let op1: u8 = 0x03;
        let result = shr(op0, op1);

        assert_eq!(result, 0x02);
    }

    #[test]
    #[should_panic]
    fn test_function_shl_should_panic_overflow() {
        let op0: u32 = 0x05;
        let op1: u8 = 0x30;
        shl(op0, op1);
    }

    #[test]
    #[should_panic]
    fn test_function_shr_should_panic_overflow() {
        let op0: u32 = 0x10;
        let op1: u8 = 0x28;
        shr(op0, op1);
    }
}
