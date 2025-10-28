use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

use crate::impl_trait;

/// Non-assigning bitwise operations.
pub trait BitOps:
    Sized
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Shl<Output = Self>
    + Shl<u32, Output = Self>
    + Shr<Output = Self>
    + Shr<u32, Output = Self>
    + Not
{
}

impl_trait!(BitOps: i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
