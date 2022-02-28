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
/// OpCode correspond à l'action à executer sur les registres.
/// ```LDW => LOAD 
/// STW => STORE
/// ADD => ADD
/// XOR => XOR
/// SHR => Shift Right
/// SHL => Shift Left
enum OpCode {
    LDW = 0x00,
    STW = 0x01,
    ADD = 0x02,
    XOR = 0x03,
    SHR = 0x04,
    SHL = 0x05
}
/// 
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
/// Additionne deux registres d'indice op0 et op1
    ///
    /// # Arguments
    ///     
    /// * `op0` - Indice du premier registre
    /// * `op1` - Indice du deuxième registre
    ///
    /// # Examples
    /// 
    /// ```
    /// r1 = add(op0, op1)
    /// ```
fn add(op0: u32, op1: u32) -> u32 {
    op0 + op1
}
/// OU exclusif sur deux registres d'indice op0 et op1
    ///
    /// # Arguments
    ///     
    /// * `op0` - Indice du premier registre
    /// * `op1` - Indice du deuxième registre
    ///
    /// # Examples
    /// 
    /// ```
    /// r1 = xor(op0, op1)
    /// ```
fn xor(op0: u32, op1: u32) -> u32 {
    op0 ^ op1
}
/// Décale les bits d'un registre vers la droite x fois
    ///
    /// # Arguments
    ///     
    /// * `op0` - Indice du registre
    /// * `x` - Nombre de bits à décaler 
    /// 
    /// # Examples
    /// 
    /// ```
    /// let x : u8 = 3
    /// let op0 : u32 = 10
    /// r1 = shift_left(op0, op1)
    /// ```
    /// Dans l'exemple 10 = 0b1010
    /// après un décalage de 3 : 0b1010000 ( = 80 en base 10)
    /// 
fn shift_left(r : u32, x : u8) -> u32 {
    return r << x;
}
/// Décale les bits d'un registre vers la gauche x fois
    ///
    /// # Arguments
    ///     
    /// * `op0` - Indice du registre
    /// * `x` - Nombre de bits à décaler 
    /// 
    /// # Examples
    /// 
    /// ```
    /// let x : u8 = 3
    /// let op0 : u32 = 10
    /// r1 = shift_right(op0, op1)
    /// ```
    /// Dans l'exemple 10 = 0b1010
    /// après un décalage de 3 : 0b1 
    /// 
fn shift_right(r : u32, x : u8) -> u32 {
    return r >> x;
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
    let mut r1: u32 = 10;
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
        OpCode::SHR => r1 = shift_right(r1,decoded_instruction.op1),
        OpCode::SHL => r1 = shift_left(r1,decoded_instruction.op1),
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
    fn test_instruction_disassemble_xor_r1_r3(){
        let insn_bytes: u32 = 0x1843;
        let insn = Instruction::disassemble(insn_bytes);
        assert_eq!(insn.opcode, OpCode::XOR);
        assert_eq!(insn.op0, 1);
        assert_eq!(insn.op1, 3);
    }
    #[test]
    fn test_instruction_disassemble_stw_r1_r3(){
        let insn_bytes: u32 = 0x1841;
        let insn = Instruction::disassemble(insn_bytes);
        assert_eq!(insn.opcode, OpCode::STW);
        assert_eq!(insn.op0, 1);
        assert_eq!(insn.op1, 3);
    }
    #[test]
    fn test_instruction_disassemble_ldw_r1_r3(){
        let insn_bytes: u32 = 0x1840;
        let insn = Instruction::disassemble(insn_bytes);
        assert_eq!(insn.opcode, OpCode::LDW);
        assert_eq!(insn.op0, 1);
        assert_eq!(insn.op1, 3);
    }
    #[test]
    fn test_instruction_disassemble_shr_r1_2(){
        let insn_bytes: u32 = 0x1844;
        let mut r1 = 20;
        let insn = Instruction::disassemble(insn_bytes);
        assert_eq!(insn.opcode, OpCode::SHR);
        assert_eq!(super::shift_right(r1, insn.op1), 2);
        r1 = 1;
        assert_eq!(super::shift_right(r1, insn.op1), 0);
    }
    #[test]
    fn test_instruction_disassemble_shl_r1_2(){
        let insn_bytes: u32 = 0x1845;
        let r1 = 20;
        let insn = Instruction::disassemble(insn_bytes);
        assert_eq!(insn.opcode, OpCode::SHL);
        assert_eq!(super::shift_left(r1, insn.op1), 160);
    }
}
