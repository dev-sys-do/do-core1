use clap::Parser;
use do_core::instruction::{Instruction, OpCode};
use do_core::{Error, MAX_REGISTER_INDEX};

#[derive(Parser)]
#[clap(version, author)]
struct DoCoreOpts {
    /// DO Core instruction
    #[clap(short, long)]
    insn: String,
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
    let op0 = decoded_instruction.op0() as usize;
    let op1 = decoded_instruction.op1() as usize;

    match decoded_instruction.opcode() {
        OpCode::ADD => registers[op0] = add(registers[op0], registers[op1])?,
        OpCode::XOR => registers[op0] = xor(registers[op0], registers[op1]),
        _ => panic!("Unknown opcode {:?}", decoded_instruction.opcode()),
    }

    dump_cpu_state("Final CPU state", &registers);

    Ok(())
}
