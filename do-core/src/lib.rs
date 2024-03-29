use crate::instruction::OpCode;

#[derive(Debug)]
pub enum Error {
    InvalidOpCode(u8),
    UnsupportedOpCode(OpCode),
    Op0OutOfRange,
    Op1OutOfRange,
    AdditionOverflow(u32, u32),
    MemoryOverflow(u32),
}

// do-core1 register indexes range from 0 to 31.
pub const MAX_REGISTER_INDEX: u8 = 31;

// Hard-code do-core memory to 1 MB
pub const MEMORY_SIZE: usize = 0x100000;

pub mod core;
pub mod instruction;
pub mod memory;
