#[allow(dead_code)]
#[derive(Debug)]
enum OpCode {
    LDW = 0x00,
    STW = 0x01,
    ADD = 0x02,
    XOR = 0x03,
}

fn add(op0: u32, op1: u32) -> u32 {
    op0 + op1
}

fn main() {
    let opcode: OpCode = OpCode::ADD;
    let mut r0: u32 = 10;
    let r1: u32 = 20;

    println!(
        "do-core-1: opcode {:?} Initial CPU state [R0:{} R1:{}]",
        opcode, r0, r1
    );

    match opcode {
        OpCode::ADD => r0 = add(r0, r1),
        _ => panic!("Unknown opcode {:?}", opcode),
    }

    println!(
        "do-core-1: opcode {:?} Final CPU state [R0:{} R1:{}]",
        opcode, r0, r1
    );
}
