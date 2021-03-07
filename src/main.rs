extern crate clap;

use clap::{App, Arg};
use do_core::instruction::{Instruction, OpCode};
use do_core::{Error, MAX_REGISTER_INDEX};

fn add(op0: u16, op1: u16) -> Result<u16, Error> {
    op0.checked_add(op1)
        .ok_or(Error::AdditionOverflow(op0, op1))
}

fn xor(op0: u16, op1: u16) -> u16 {
    op0 ^ op1
}

fn dump_cpu_state(preamble: &str, registers: &[u16; MAX_REGISTER_INDEX as usize + 1]) {
    println!("do-core1: {}:", preamble);
    for (index, register) in registers.iter().enumerate() {
        println!("\tR{}: {:#x?}", index, *register);
    }
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
    let mut registers = [0u16; MAX_REGISTER_INDEX as usize + 1];
    // Arbitrary initial registers value.
    // Registers will eventually be initialized through memory loads.
    for (index, register) in registers.iter_mut().enumerate() {
        *register = index as u16 * 0x10;
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
