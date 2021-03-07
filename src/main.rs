use clap::Parser;
use do_core::instruction::OpCode;
use do_core::Error;

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

// do-core1 register indexes range from 0 to 31.
const MAX_REGISTER_INDEX: u8 = 31;

impl Instruction {
    // Instruction constructor, a.k.a. disassembler.
    fn disassemble(insn: u32) -> Result<Instruction, Error> {
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
}

fn add(op0: u32, op1: u32) -> Result<u32, Error> {
    op0.checked_add(op1)
        .ok_or(Error::AdditionOverflow(op0, op1))
}

fn xor(op0: u32, op1: u32) -> u32 {
    op0 ^ op1
}

fn dump_cpu_state(preamble: &str, registers: &[u32; MAX_REGISTER_INDEX as usize + 1]) {
    println!("do-core1: {}:", preamble);
    for (index, register) in registers.iter().enumerate() {
        println!("\tR{}: {:#x?}", index, *register);
    }
}

fn main() -> Result<(), Error> {
    let opts: DoCoreOpts = DoCoreOpts::parse();
    let insn = u32::from_str_radix(opts.insn.trim_start_matches("0x"), 16).unwrap();
    let mut registers = [0u32; MAX_REGISTER_INDEX as usize + 1];
    // Arbitrary initial registers value.
    // Registers will eventually be initialized through memory loads.
    for (index, register) in registers.iter_mut().enumerate() {
        *register = index as u32 * 0x10;
    }

    dump_cpu_state("Initial CPU state", &registers);

    let decoded_instruction = Instruction::disassemble(insn)?;
    println!(
        "do-core-1: instruction decoded into {:?}",
        decoded_instruction
    );
    let op0 = decoded_instruction.op0 as usize;
    let op1 = decoded_instruction.op1 as usize;

    match decoded_instruction.opcode {
        OpCode::ADD => registers[op0] = add(registers[op0], registers[op1])?,
        OpCode::XOR => registers[op0] = xor(registers[op0], registers[op1]),

        _ => panic!("Unknown opcode {:?}", decoded_instruction.opcode),
    }

    dump_cpu_state("Final CPU state", &registers);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Error, Instruction, OpCode};

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
