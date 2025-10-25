use crate::{Binary, MinMax, Number};

pub trait Integer: Binary + MinMax + Number {}

macro_rules! impl_integer {
    ($($t:ty )*) => {
        $(
            impl Integer for $t {

            }
        )*
    };
}

impl_integer!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
