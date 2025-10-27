use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

pub trait NumAssignOps: Sized + AddAssign + SubAssign + MulAssign + DivAssign + RemAssign {}

crate::impl_trait!(NumAssignOps: i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64);
