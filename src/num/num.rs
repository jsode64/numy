use crate::MinMax;

use super::{NumAssignOps, NumOps};

pub trait Num: Copy + PartialEq + PartialOrd + MinMax + NumAssignOps + NumOps {
    const ZERO: Self;

    const ONE: Self;

    const TWO: Self;
}

macro_rules! impl_num {
    ($($t:ty)*) => {
        $(
            impl Num for $t {
                const ZERO: Self = 0 as Self;

                const ONE: Self = 1 as Self;

                const TWO: Self = 2 as Self;
            }
        )*
    };
}

impl_num!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64);
