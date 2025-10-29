use crate::MinMax;

use super::{NumAssignOps, NumOps, NumOrd};

/// Numerical types.
///
/// Exposes:
/// - Non-assignming arithmetic operations
/// - Assigning arithmetic operations
/// - Partial comparisons
pub trait Num: Copy + PartialEq + MinMax + NumAssignOps + NumOps + NumOrd {
    const ZERO: Self;

    const ONE: Self;

    const TWO: Self;

    /// Returns the difference of `self` and `other`.
    ///
    /// The purpose of this function is to safely subtract two types without
    /// risk of underflow for unsigned integers.
    ///
    /// For signed types (`i32`, `f64`, etc.) returns `self - other`.
    ///
    /// For unsigned types (`uN`) returns `self.abs_diff(other)`.
    #[cfg(feature = "ex")]
    #[must_use]
    fn diff(self, other: Self) -> Self;

    /// Returns `self`'s absolute value.
    ///
    /// The purpose of this function is to guarantee a non-negative number for
    /// generic `Num` types regardless of signage.
    ///
    /// For signed types (`i32`, `f64`, etc.) returns `self.abs()`.
    ///
    /// For unsigned types (`uN`) returns `self`.
    #[cfg(feature = "ex")]
    #[must_use]
    fn abs(self) -> Self;
}

macro_rules! impl_num_u {
    ($($t:ty)*) => {
        $(
            impl Num for $t {
                const ZERO: Self = 0 as Self;

                const ONE: Self = 1 as Self;

                const TWO: Self = 2 as Self;

                #[cfg(feature = "ex")]
                #[inline(always)]
                fn diff(self, other: Self) -> Self {
                    self.abs_diff(other)
                }

                #[cfg(feature = "ex")]
                #[inline(always)]
                fn abs(self) -> Self {
                    self
                }
            }
        )*
    };
}

impl_num_u!(u8 u16 u32 u64 u128 usize);

macro_rules! impl_num_s {
    ($($t:ty)*) => {
        $(
            impl Num for $t {
                const ZERO: Self = 0 as Self;

                const ONE: Self = 1 as Self;

                const TWO: Self = 2 as Self;

                #[cfg(feature = "ex")]
                #[inline(always)]
                fn diff(self, other: Self) -> Self {
                    self - other
                }

                #[cfg(feature = "ex")]
                #[inline(always)]
                fn abs(self) -> Self {
                    self.abs()
                }
            }
        )*
    };
}

impl_num_s!(i8 i16 i32 i64 i128 isize f32 f64);
