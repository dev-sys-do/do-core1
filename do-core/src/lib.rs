#[derive(Debug)]
pub enum Error {
    InvalidOpCode(u8),
    Op0OutOfRange,
    Op1OutOfRange,
    AdditionOverflow(u16, u16),
}

// do-core register indexes range from 0 to 7.
pub const MAX_REGISTER_INDEX: u8 = 7;

pub mod instruction;
