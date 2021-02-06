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

fn xor(op0: u32, op1: u32) -> u32 {
    op0 ^ op1
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
    let insn: u32 = 0x1842;
    let mut r1: u32 = 20;
    let r3: u32 = 12;

    println!(
        "do-core-1: instruction {:#x?} Initial CPU state [R1:{:#x?} R3:{:#x?}]",
        insn, r1, r3
    );

    // Keep the first 6 bits only
    let opcode = (insn & 0x3f) as u8;

    // Shift right by 6, keep only the first 5 bits.
    let op0 = ((insn >> 6) & 0x1f) as u8;

    // Shift right by 11, keep only the first 5 bits.
    let op1 = ((insn >> 11) & 0x1f) as u8;

    println!(
        "do-core-1: instruction decoded into [opcode:{:?} op0:{} op1:{}]",
        opcode, op0, op1
    );

    match opcode {
        OpCode::ADD => r1 = add(r1, r3),
        OpCode::XOR => r1 = xor(r1, r3),
        _ => panic!("Unknown opcode {:?}", opcode),
    }

    println!(
        "do-core-1: instruction {:#x?} Final CPU state [R1:{:#x?} R3:{:#x?}]",
        insn, r1, r3
    );
}
