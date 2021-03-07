#[derive(Debug)]
pub enum Error {
    InvalidOpCode(u8),
    Op0OutOfRange,
    Op1OutOfRange,
    AdditionOverflow(u32, u32),
}

pub mod instruction;
