use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};

use crate::impl_trait;

pub trait BitAssignOps:
    Sized
    + BitAndAssign
    + BitOrAssign
    + BitXorAssign
    + ShlAssign
    + ShlAssign<u32>
    + ShrAssign
    + ShrAssign<u32>
{
}

impl_trait!(BitAssignOps: i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
