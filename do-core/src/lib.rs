use crate::instruction::OpCode;

#[derive(Debug)]
pub enum Error {
    InvalidOpCode(u8),
    UnsupportedOpCode(OpCode),
    Op0OutOfRange,
    Op1OutOfRange,
    AdditionOverflow(u16, u16),
    MemoryOverflow(u16),
}

// do-core register indexes range from 0 to 7.
pub const MAX_REGISTER_INDEX: u8 = 7;

// do-core only support 4K of memory
pub const MEMORY_SIZE: usize = 0x1000;

pub mod core;
pub mod instruction;
pub mod memory;
