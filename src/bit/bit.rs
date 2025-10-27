use crate::impl_trait;

use super::{BitAssignOps, BitOps};

pub trait Bit: Copy + BitAssignOps + BitOps {}

impl_trait!(Bit: i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
