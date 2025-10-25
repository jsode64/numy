pub trait MinMax {
    const MIN: Self;

    const MAX: Self;
}

macro_rules! impl_min_max {
    ($($t:ty)*) => {
        $(
            impl MinMax for $t {
                const MIN: Self = Self::MIN;

                const MAX: Self = Self::MAX;
            }
        )*
    };
}

impl_min_max!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64);
