use crate::{Integer, Signed, UnsignedInteger};

pub trait SignedInteger: Integer + Signed {
    /// The unsigned integer type with the same size.
    type U: UnsignedInteger;

    /// Checked absolute value.
    #[must_use]
    fn checked_abs(self) -> Option<Self>;

    /// Saturating absolute value.
    #[must_use]
    fn saturating_abs(self) -> Self;

    /// Wrapping (modular) absolute value.
    #[must_use]
    fn wrapping_abs(self) -> Self;

    /// Computes the absolute value of `self`, with overflow information.
    #[must_use]
    fn overflowing_abs(self) -> (Self, bool);

    /// Computes the absolute value of `self` without any wrapping or panicking.
    #[must_use]
    fn unsigned_abs(self) -> Self::U;

    /// Checked negation.
    #[must_use]
    fn checked_neg(self) -> Option<Self>;

    /// Saturating integer negation.
    #[must_use]
    fn saturating_neg(self) -> Self;

    /// Wrapping (modular) negation.
    #[must_use]
    fn wrapping_neg(self) -> Self;

    /// Negates self, overflowing if this is equal to the minimum value.
    #[must_use]
    fn overflowing_neg(self) -> (Self, bool);

    /// Checked addition with an unsigned integer.
    #[must_use]
    fn checked_add_unsigned(self, rhs: Self::U) -> Option<Self>;

    /// Saturating addition with an unsigned integer.
    #[must_use]
    fn saturating_add_unsigned(self, rhs: Self::U) -> Self;

    /// Wrapping (modular) addition with an unsigned integer.
    #[must_use]
    fn wrapping_add_unsigned(self, rhs: Self::U) -> Self;

    /// Calculates `self` + `rhs` with an unsigned `rhs`.
    #[must_use]
    fn overflowing_add_unsigned(self, rhs: Self::U) -> (Self, bool);

    /// Checked subtraction with an unsigned integer.
    #[must_use]
    fn checked_sub_unsigned(self, rhs: Self::U) -> Option<Self>;

    /// Saturating subtraction with an unsigned integer.
    #[must_use]
    fn saturating_sub_unsigned(self, rhs: Self::U) -> Self;

    /// Wrapping (modular) subtraction with an unsigned integer.
    #[must_use]
    fn wrapping_sub_unsigned(self, rhs: Self::U) -> Self;

    /// Calculates `self` - `rhs` with an unsigned `rhs`.
    #[must_use]
    fn overflowing_sub_unsigned(self, rhs: Self::U) -> (Self, bool);

    /// Returns the bit pattern of `self` reinterpreted as an unsigned integer of the same size.
    #[must_use]
    fn cast_unsigned(self) -> Self::U;

    /// Returns a number representing sign of `self`.
    #[must_use]
    fn signum(self) -> Self;

    /// Returns the integer square root of a number.
    #[must_use]
    fn isqrt(self) -> Self;

    /// Checked integer square root.
    #[must_use]
    fn checked_isqrt(self) -> Option<Self>;

    /// Calculates the quotient of Euclidean division of `self` by `rhs`.
    #[must_use]
    fn div_euclid(self, rhs: Self) -> Self;

    /// Calculates the least nonnegative remainder of `self (mod rhs)`.
    #[must_use]
    fn rem_euclid(self, rhs: Self) -> Self;

    /// Returns the logarithm of the number with respect to an arbitrary base, rounded down.
    #[must_use]
    fn ilog(self, base: Self) -> u32;

    /// Returns the base 2 logarithm of the number, rounded down.
    #[must_use]
    fn ilog2(self) -> u32;

    /// Returns the base 10 logarithm of the number, rounded down.
    #[must_use]
    fn ilog10(self) -> u32;

    /// Checked integer logarithm to base.
    #[must_use]
    fn checked_ilog(self, base: Self) -> Option<u32>;

    /// Checked base 2 logarithm.
    #[must_use]
    fn checked_ilog2(self) -> Option<u32>;

    /// Checked base 10 logarithm.
    #[must_use]
    fn checked_ilog10(self) -> Option<u32>;
}

macro_rules! impl_signed_integer {
    ($($t:ty, $u:ty);* $(;)*) => {
        $(
            impl SignedInteger for $t {
                type U = $u;

                #[inline(always)]
                fn checked_abs(self) -> Option<Self> {
                    Self::checked_abs(self)
                }

                #[inline(always)]
                fn saturating_abs(self) -> Self {
                    Self::saturating_abs(self)
                }

                #[inline(always)]
                fn wrapping_abs(self) -> Self {
                    Self::wrapping_abs(self)
                }

                #[inline(always)]
                fn overflowing_abs(self) -> (Self, bool) {
                    Self::overflowing_abs(self)
                }

                #[inline(always)]
                fn unsigned_abs(self) -> Self::U {
                    Self::unsigned_abs(self)
                }

                #[inline(always)]
                fn checked_neg(self) -> Option<Self> {
                    Self::checked_neg(self)
                }

                #[inline(always)]
                fn saturating_neg(self) -> Self {
                    Self::saturating_neg(self)
                }

                #[inline(always)]
                fn wrapping_neg(self) -> Self {
                    Self::wrapping_neg(self)
                }

                #[inline(always)]
                fn overflowing_neg(self) -> (Self, bool) {
                    Self::overflowing_neg(self)
                }

                #[inline(always)]
                fn checked_add_unsigned(self, rhs: Self::U) -> Option<Self> {
                    Self::checked_add_unsigned(self, rhs)
                }

                #[inline(always)]
                fn saturating_add_unsigned(self, rhs: Self::U) -> Self {
                    Self::saturating_add_unsigned(self, rhs)
                }

                #[inline(always)]
                fn wrapping_add_unsigned(self, rhs: Self::U) -> Self {
                    Self::wrapping_add_unsigned(self, rhs)
                }

                #[inline(always)]
                fn overflowing_add_unsigned(self, rhs: Self::U) -> (Self, bool) {
                    Self::overflowing_add_unsigned(self, rhs)
                }

                #[inline(always)]
                fn checked_sub_unsigned(self, rhs: Self::U) -> Option<Self> {
                    Self::checked_sub_unsigned(self, rhs)
                }

                #[inline(always)]
                fn saturating_sub_unsigned(self, rhs: Self::U) -> Self {
                    Self::saturating_sub_unsigned(self, rhs)
                }

                #[inline(always)]
                fn wrapping_sub_unsigned(self, rhs: Self::U) -> Self {
                    Self::wrapping_sub_unsigned(self, rhs)
                }

                #[inline(always)]
                fn overflowing_sub_unsigned(self, rhs: Self::U) -> (Self, bool) {
                    Self::overflowing_sub_unsigned(self, rhs)
                }

                #[inline(always)]
                fn cast_unsigned(self) -> Self::U {
                    Self::cast_unsigned(self)
                }

                #[inline(always)]
                fn signum(self) -> Self {
                    Self::signum(self)
                }

                #[inline(always)]
                fn isqrt(self) -> Self {
                    Self::isqrt(self)
                }

                #[inline(always)]
                fn checked_isqrt(self) -> Option<Self> {
                    Self::checked_isqrt(self)
                }

                #[inline(always)]
                fn div_euclid(self, rhs: Self) -> Self {
                    Self::div_euclid(self, rhs)
                }

                #[inline(always)]
                fn rem_euclid(self, rhs: Self) -> Self {
                    Self::rem_euclid(self, rhs)
                }

                #[inline(always)]
                fn ilog(self, base: Self) -> u32 {
                    Self::ilog(self, base)
                }

                #[inline(always)]
                fn ilog2(self) -> u32 {
                    Self::ilog2(self)
                }

                #[inline(always)]
                fn ilog10(self) -> u32 {
                    Self::ilog10(self)
                }

                #[inline(always)]
                fn checked_ilog(self, base: Self) -> Option<u32> {
                    Self::checked_ilog(self, base)
                }

                #[inline(always)]
                fn checked_ilog2(self) -> Option<u32> {
                    Self::checked_ilog2(self)
                }

                #[inline(always)]
                fn checked_ilog10(self) -> Option<u32> {
                    Self::checked_ilog10(self)
                }
            }
        )*
    };
}

impl_signed_integer!(
    i8, u8;
    i16, u16;
    i32, u32;
    i64, u64;
    i128, u128;
    isize, usize;
);
