use std::ops::Neg;

use crate::Number;

pub trait Signed: Neg<Output = Self> + Number {
    /// The type's negative one (-1) value.
    const NEG_ONE: Self;

    /// The value's absolute value.
    #[must_use]
    fn abs(self) -> Self;

    /// Returns `true` if `self` is negative and `false` if not.
    #[must_use]
    fn is_negative(self) -> bool;

    /// Returns `true` if `self` is positive and `false` if not.
    #[must_use]
    fn is_positive(self) -> bool;
}

macro_rules! impl_signed {
    ($($t:ty)*) => {
        $(
            impl Signed for $t {
                const NEG_ONE: Self = -1 as Self;

                #[inline(always)]
                fn abs(self) -> Self {
                    self.abs()
                }

                #[inline(always)]
                fn is_negative(self) -> bool {
                    self < 0 as Self
                }

                #[inline(always)]
                fn is_positive(self) -> bool {
                    self > 0 as Self
                }
            }
        )*
    };
}

impl_signed!(i8 i16 i32 i64 i128 isize f32 f64);
