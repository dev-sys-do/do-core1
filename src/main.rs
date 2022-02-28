/// An instruction must have an opcode to defined which operation to run on op0 and op1
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

/// Represent an operation code
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum OpCode {
    LDW = 0x00,
    STW = 0x01,
    ADD = 0x02,
    XOR = 0x03,
    SHL = 0x04,
    SHR = 0x05,
}

impl OpCode {
    /// Provides a `from_u8` function to convert an opcode from u8 to OpCode
    fn from_u8(opcode: u8) -> OpCode {
        match opcode {
            0x00 => OpCode::LDW,
            0x01 => OpCode::STW,
            0x02 => OpCode::ADD,
            0x03 => OpCode::XOR,
            0x04 => OpCode::SHL,
            0x05 => OpCode::SHR,
            _ => panic!("Unknown opcode {:?}", opcode),
        }
    }
}

/// Add `op0` with `op1`
fn add(op0: u32, op1: u32) -> u32 {
    op0 + op1
}

/// Perform a xor operation on `op0` and `op1`
fn xor(op0: u32, op1: u32) -> u32 {
    op0 ^ op1
}

/// Perform a left shifting from `op0` of `op1` bits
fn shift_left(op0: u32, op1: u32) -> u32 {
    if op1 < 32 {
        op0 << op1
    } else {
        panic!("Attempt to shift left by {:?} which would overflow", op1);
    }
}

/// Perform a right shifting from `op0` of `op1` bits
fn shift_right(op0: u32, op1: u32) -> u32 {
    if op1 < 32 {
        op0 >> op1
    } else {
        panic!("Attempt to shift right by {:?} which would overflow", op1);
    }
}

/// Main entrypoint
fn main() {
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
    let insn: u32 = 0x1842;
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
        OpCode::SHL => r1 = shift_left(r1, r3),
        OpCode::SHR => r1 = shift_right(r1, r3),
        _ => panic!("Unknown opcode {:?}", decoded_instruction.opcode),
    }

    println!(
        "do-core-1: instruction {:#x?} Final CPU state [R1:{:#x?} R3:{:#x?}]",
        insn, r1, r3
    );
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, OpCode, add, xor, shift_left, shift_right};

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
    fn test_instruction_disassemble_xor_r1_r3() {
        let insn_bytes: u32 = 0x1843;
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::XOR);
        assert_eq!(insn.op0, 1);
        assert_eq!(insn.op1, 3);
    }

    #[test]
    fn test_instruction_disassemble_shift_left_r1_r3() {
        let insn_bytes: u32 = 0x1844;
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::SHL);
        assert_eq!(insn.op0, 1);
        assert_eq!(insn.op1, 3);
    }

    #[test]
    fn test_instruction_disassemble_shift_right_r1_r3() {
        let insn_bytes: u32 = 0x1845;
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::SHR);
        assert_eq!(insn.op0, 1);
        assert_eq!(insn.op1, 3);
    }

    #[test]
    fn test_add_op0_op1() {
        let op0: u32 = 1;
        let op1: u32 = 2;
        let result: u32 = add(op0, op1);

        assert_eq!(result, 3);
    }

    #[test]
    fn test_xor_op0_op1() {
        let op0: u32 = 1;
        let op1: u32 = 2;
        let result: u32 = xor(op0, op1);

        assert_eq!(result, 3);
    }

    #[test]
    fn test_shift_left_ok_op0_op1() {
        let op0: u32 = 1;
        let op1: u32 = 2;
        let result: u32 = shift_left(op0, op1);

        assert_eq!(result, 4);
    }

    #[test]
    fn test_shift_right_ok_op0_op1() {
        let op0: u32 = 2;
        let op1: u32 = 1;
        let result: u32 = shift_right(op0, op1);

        assert_eq!(result, 1);
    }

    #[test]
    #[should_panic]
    fn test_shift_left_panic_op0_op1() {
        let op0: u32 = 1;
        let op1: u32 = 0xff;
        shift_left(op0, op1);
    }

    #[test]
    #[should_panic]
    fn test_shift_right_panic_op0_op1() {
        let op0: u32 = 1;
        let op1: u32 = 0xff;
        shift_right(op0, op1);
    }
}
