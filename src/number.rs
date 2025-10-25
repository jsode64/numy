use crate::MinMax;
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

pub trait Number:
    Clone
    + Copy
    + Debug
    + PartialEq
    + PartialOrd
    + Sized
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + MinMax
{
    /// The number's zero (0) value.
    const ZERO: Self;

    /// The number's one (1) value.
    const ONE: Self;

    /// The number's two (2) value.
    const TWO: Self;
}

macro_rules! impl_number {
    ($($t:ty)*) => {
        $(
            impl Number for $t {
                const ZERO: Self = 0 as Self;

                const ONE: Self = 1 as Self;

                const TWO: Self = 2 as Self;
            }
        )*
    };
}

impl_number!(
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
    f32 f64
);
