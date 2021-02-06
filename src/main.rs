extern crate clap;
use clap::{App, Arg};

#[derive(Debug)]
struct Instruction {
    opcode: OpCode,
    op0: u8,
    op1: u8,
}

impl Instruction {
    // Instruction constructor, a.k.a. disassembler.
    fn disassemble(insn: u16) -> Instruction {
        let opcode = OpCode::from_u8((insn >> 8) as u8);
        let op0 = ((insn & 0xf0) >> 4) as u8;
        let op1: u8 = (insn & 0xf) as u8;

        Instruction { opcode, op0, op1 }
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
    fn from_u8(opcode: u8) -> OpCode {
        match opcode {
            0x00 => OpCode::LD,
            0x01 => OpCode::ST,
            0x02 => OpCode::ADD,
            0x03 => OpCode::XOR,
            _ => panic!("Unknown opcode {:?}", opcode),
        }
    }
}

fn add(op0: u16, op1: u16) -> u16 {
    op0 + op1
}

fn xor(op0: u16, op1: u16) -> u16 {
    op0 ^ op1
}

fn main() {
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

    let decoded_instruction = Instruction::disassemble(insn);
    println!(
        "do-core-1: instruction decoded into {:?}",
        decoded_instruction
    );

    match decoded_instruction.opcode {
        OpCode::ADD => r0 = add(r0, r1),
        OpCode::XOR => r0 = xor(r0, r1),
        _ => panic!("Unknown opcode {:?}", decoded_instruction.opcode),
    }

    println!(
        "do-core-1: instruction {:#x?} Final CPU state [R0:{:#x?} R1:{:#x?}]",
        insn, r0, r1
    );
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, OpCode};

    #[test]
    fn test_instruction_disassemble_add_r0_r1() {
        let insn_bytes: u16 = 0x201;
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::ADD);
        assert_eq!(insn.op0, 0);
        assert_eq!(insn.op1, 1);
    }

    #[test]
    fn test_instruction_disassemble_add_r7_r2() {
        let insn_bytes: u16 = 0x272;
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::ADD);
        assert_eq!(insn.op0, 7);
        assert_eq!(insn.op1, 2);
    }

    #[test]
    fn test_instruction_disassemble_ld_r0_r1() {
        let insn_bytes: u16 = 0x01;
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::LD);
        assert_eq!(insn.op0, 0);
        assert_eq!(insn.op1, 1);
    }

    #[test]
    fn test_instruction_disassemble_xor_r2_r3() {
        let insn_bytes: u16 = 0x323;
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::XOR);
        assert_eq!(insn.op0, 2);
        assert_eq!(insn.op1, 3);
    }

    #[test]
    fn test_instruction_disassemble_st_r5_r0() {
        let insn_bytes: u16 = 0x150;
        let insn = Instruction::disassemble(insn_bytes);

        assert_eq!(insn.opcode, OpCode::ST);
        assert_eq!(insn.op0, 5);
        assert_eq!(insn.op1, 0);
    }
}
