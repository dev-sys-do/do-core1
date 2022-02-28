use clap::Parser;

/// Contains the operation code (`OpCode`) and two operands : `op0` and `op1`
#[derive(Debug)]
struct Instruction {
    opcode: OpCode,
    op0: u8,
    op1: u8,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// instruction code to parse and execute, ex : 0x1842
    #[clap(short, long)]
    instruction: String,
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

/// The list of all usable OpCode and their equivalent in base 16. 
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

/// Return the sum of `op0` and `op1`
fn add(op0: u32, op1: u32) -> u32 {
    op0 + op1
}

/// Return the value of a binary xor between `op0` and `op1`
fn xor(op0: u32, op1: u32) -> u32 {
    op0 ^ op1
}

/// Return `op0` value with a shift of `shift` bits to the right 
fn shr(op0: u32, shift: u32) -> u32 {
    op0 >> shift
}


/// Return `op0` value with a shift of `shift` bits to the left 
fn shl(op0: u32, shift: u32) -> u32 {
    op0 << shift
}

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
    let args = Args::parse();
    let insn: u32 = u32::from_str_radix(args.instruction.trim_start_matches("0x"), 16).unwrap();
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
    fn test_instruction_disassemble_add_r1_r3() {
        let insn_bytes: u32 = 0x1842;
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::ADD);
        assert_eq!(insn.op0, 1);
        assert_eq!(insn.op1, 3);
    }


    #[test]
    fn test_instruction_disassemble_xor_r1_r2(){
        let insn_bytes: u32 = 0x1043; // 0001 0000 0100 0011
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::XOR);
        assert_eq!(insn.op0, 1);
        assert_eq!(insn.op1, 2);
    }
    

    #[test]
    fn test_instruction_disassemble_lwd_r5_r4(){
        let insn_bytes: u32 = 0x2140; // 0010 0001 0100 0000
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::LDW);
        assert_eq!(insn.op0, 5);
        assert_eq!(insn.op1, 4);
    }


    #[test]
    fn test_instruction_disassemble_stw_r1_r7(){
        let insn_bytes: u32 = 0x3841; // 0011 1000 0100 0001
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::STW);
        assert_eq!(insn.op0, 1);
        assert_eq!(insn.op1, 7);
    }

    #[test]
    fn test_instruction_disassemble_shr_r6_r1(){
        let insn_bytes: u32 = 0x0984; // 0000 1001 1000 0100
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::SHR);
        assert_eq!(insn.op0, 6);
        assert_eq!(insn.op1, 1);
    }

    #[test]
    fn test_instruction_disassemble_shl_r1_r7(){
        let insn_bytes: u32 = 0x3845; // 0011 1000 0100 0101
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::SHL);
        assert_eq!(insn.op0, 1);
        assert_eq!(insn.op1, 7);
    }

    #[test]
    fn test_shr_6_1(){
        assert_eq!(crate::shr(6, 1), 3);
    }

    #[test]
    fn test_shr_6_2(){
        assert_eq!(crate::shr(6, 2), 1);
    }

    #[test]
    fn test_shr_6_3(){
        assert_eq!(crate::shr(6, 3), 0);
    }

    #[test]
    #[should_panic]
    fn test_shr_6_32(){
        assert_eq!(crate::shr(6, 32), 0);
    }

    #[test]
    fn test_shl_1_7(){
        assert_eq!(crate::shl(1, 7), 128);
    }

    #[test]
    fn test_shl_1_8(){
        assert_eq!(crate::shl(1, 8), 256);
    }

    #[test]
    #[should_panic]
    fn test_shl_1_32(){
        crate::shl(1, 32);
    }

}
