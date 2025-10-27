use std::ops::{Add, Div, Mul, Rem, Sub};

use crate::impl_trait;

pub trait NumOps:
    Sized
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
{
}

impl_trait!(NumOps: i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64);
