use crate::Num;

/// Provides utility functions for simpler usage of the general `Numy` types.
pub trait NumUtil: Num {
    /// Returns the difference between the two values.
    ///
    /// For signed types (`i32`, `f64`, etc.), returns `self - other`.
    ///
    /// For unsigned types (`u8`, `u32`, etc.), returns `self.abs_diff(other)`.
    #[must_use]
    fn diff(self, other: Self) -> Self;

    /// The value as a positive (absolute value).
    ///
    /// For signed types (`i32`, `f64`, etc.), returns `self.abs()`.
    ///
    /// For unsigned types (`u8`, `u32`, etc.), returns `self`.
    #[must_use]
    fn abs(self) -> Self;
}

macro_rules! impl_num_util_signed {
    ($($t:ty)*) => {
        $(
            impl NumUtil for $t {
                #[inline(always)]
                fn diff(self, other: Self) -> Self {
                    self - other
                }

                #[inline(always)]
                fn abs(self) -> Self {
                    self.abs()
                }
            }
        )*
    };
}

impl_num_util_signed!(i8 i16 i32 i64 i128 isize f32 f64);

macro_rules! impl_num_util_unsigned {
    ($($t:ty)*) => {
        $(
            impl NumUtil for $t {
                #[inline(always)]
                fn diff(self, other: Self) -> Self {
                    self.abs_diff(other)
                }

                #[inline(always)]
                fn abs(self) -> Self {
                    self
                }
            }
        )*
    };
}

impl_num_util_unsigned!(u8 u16 u32 u64 u128 usize);
