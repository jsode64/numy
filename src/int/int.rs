use crate::{Bit, Num};

/// Integer types.
///
/// Exposes:
/// - `Num` trait
/// - Bitwise operations
/// - Full comparison
/// - Integer functionality
///
/// See [`i32`] or [`u32`].
///
/// [`i32`]: https://doc.rust-lang.org/std/primitive.i32.html
/// [`u32`]: https://doc.rust-lang.org/std/primitive.u32.html
pub trait Int: Eq + Ord + Bit + Num {
    /// The size of this integer type in bits.
    const BITS: u32;

    /// Returns the number of ones in the binary representation of `self`.
    #[must_use]
    fn count_ones(self) -> u32;

    /// Returns the number of zeros in the binary representation of `self`.
    #[must_use]
    fn count_zeros(self) -> u32;

    /// Returns the number of leading zeros in the binary representation of `self`.
    #[must_use]
    fn leading_zeros(self) -> u32;

    /// Returns the number of trailing zeros in the binary representation of `self`.
    #[must_use]
    fn trailing_zeros(self) -> u32;

    /// Returns the number of leading ones in the binary representation of `self`.
    #[must_use]
    fn leading_ones(self) -> u32;

    /// Returns the number of trailing ones in the binary representation of `self`.
    #[must_use]
    fn trailing_ones(self) -> u32;

    /// Shifts the bits to the left by a specified amount, `n`, wrapping the truncated bits to the end of the resulting integer.
    #[must_use]
    fn rotate_left(self, n: u32) -> Self;

    /// Shifts the bits to the right by a specified amount, `n`, wrapping the truncated bits to the beginning of the resulting integer.
    #[must_use]
    fn rotate_right(self, n: u32) -> Self;

    /// Reverses the byte order of the integer.
    #[must_use]
    fn swap_bytes(self) -> Self;

    /// Reverses the order of bits in the integer.
    #[must_use]
    fn reverse_bits(self) -> Self;

    /// Converts an integer from big endian to the target's endianness.
    #[must_use]
    fn from_be(x: Self) -> Self;

    /// Converts an integer from little endian to the target's endianness.
    #[must_use]
    fn from_le(x: Self) -> Self;

    /// Converts `self` to big endian from the target's endianness.
    #[must_use]
    fn to_be(self) -> Self;

    /// Converts `self` to little endian from the target's endianness.
    #[must_use]
    fn to_le(self) -> Self;

    /// Checked integer addition.
    #[must_use]
    fn checked_add(self, rhs: Self) -> Option<Self>;

    /// Checked integer subtraction.
    #[must_use]
    fn checked_sub(self, rhs: Self) -> Option<Self>;

    /// Checked integer multiplication.
    #[must_use]
    fn checked_mul(self, rhs: Self) -> Option<Self>;

    /// Checked integer division.
    #[must_use]
    fn checked_div(self, rhs: Self) -> Option<Self>;

    /// Checked Euclidean division.
    #[must_use]
    fn checked_div_euclid(self, rhs: Self) -> Option<Self>;

    /// Checked integer remainder.
    #[must_use]
    fn checked_rem(self, rhs: Self) -> Option<Self>;

    /// Checked Euclidean remainder.
    #[must_use]
    fn checked_rem_euclid(self, rhs: Self) -> Option<Self>;

    /// Checked shift left.
    #[must_use]
    fn checked_shl(self, rhs: u32) -> Option<Self>;

    /// Checked shift right.
    #[must_use]
    fn checked_shr(self, rhs: u32) -> Option<Self>;

    /// Checked exponentiation.
    #[must_use]
    fn checked_pow(self, exp: u32) -> Option<Self>;

    /// Saturating integer addition.
    #[must_use]
    fn saturating_add(self, rhs: Self) -> Self;

    /// Saturating integer subtraction.
    #[must_use]
    fn saturating_sub(self, rhs: Self) -> Self;

    /// Saturating integer multiplication.
    #[must_use]
    fn saturating_mul(self, rhs: Self) -> Self;

    /// Saturating integer division.
    #[must_use]
    fn saturating_div(self, rhs: Self) -> Self;

    /// Saturating integer exponentiation.
    #[must_use]
    fn saturating_pow(self, exp: u32) -> Self;

    /// Wrapping (modular) addition.
    #[must_use]
    fn wrapping_add(self, rhs: Self) -> Self;

    /// Wrapping (modular) subtraction.
    #[must_use]
    fn wrapping_sub(self, rhs: Self) -> Self;

    /// Wrapping (modular) multiplication.
    #[must_use]
    fn wrapping_mul(self, rhs: Self) -> Self;

    /// Wrapping (modular) division.
    #[must_use]
    fn wrapping_div(self, rhs: Self) -> Self;

    /// Wrapping Euclidean division.
    #[must_use]
    fn wrapping_div_euclid(self, rhs: Self) -> Self;

    /// Wrapping (modular) remainder.
    #[must_use]
    fn wrapping_rem(self, rhs: Self) -> Self;

    /// Wrapping Euclidean remainder.
    #[must_use]
    fn wrapping_rem_euclid(self, rhs: Self) -> Self;

    /// Panic-free bitwise shift-left; yields `self << mask(rhs)`, where `mask` removes any high-order bits of `rhs` that would cause the shift to exceed the bitwidth of the type.
    #[must_use]
    fn wrapping_shl(self, rhs: u32) -> Self;

    /// Panic-free bitwise shift-right; yields `self >> mask(rhs)`, where `mask` removes any high-order bits of `rhs` that would cause the shift to exceed the bitwidth of the type.
    #[must_use]
    fn wrapping_shr(self, rhs: u32) -> Self;

    /// Wrapping (modular) exponentiation.
    #[must_use]
    fn wrapping_pow(self, exp: u32) -> Self;

    /// Calculates `self` + `rhs`.
    #[must_use]
    fn overflowing_add(self, rhs: Self) -> (Self, bool);

    /// Calculates `self` - `rhs`.
    #[must_use]
    fn overflowing_sub(self, rhs: Self) -> (Self, bool);

    /// Calculates the multiplication of `self` and `rhs`.
    #[must_use]
    fn overflowing_mul(self, rhs: Self) -> (Self, bool);

    /// Calculates the divisor when `self` is divided by `rhs`.
    #[must_use]
    fn overflowing_div(self, rhs: Self) -> (Self, bool);

    /// Calculates the quotient of Euclidean division `self.div_euclid(rhs)`.
    #[must_use]
    fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool);

    /// Calculates the remainder when `self` is divided by `rhs`.
    #[must_use]
    fn overflowing_rem(self, rhs: Self) -> (Self, bool);

    /// Calculates the remainder `self.rem_euclid(rhs)` as if by Euclidean division.
    #[must_use]
    fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool);

    /// Shifts self left by `rhs` bits.
    #[must_use]
    fn overflowing_shl(self, rhs: u32) -> (Self, bool);

    /// Shifts self right by `rhs` bits.
    #[must_use]
    fn overflowing_shr(self, rhs: u32) -> (Self, bool);

    /// Raises self to the power of `exp`, using exponentiation by squaring.
    #[must_use]
    fn overflowing_pow(self, exp: u32) -> (Self, bool);

    /// Raises self to the power of `exp`, using exponentiation by squaring.
    #[must_use]
    fn pow(self, exp: u32) -> Self;

    /// Calculates the quotient of Euclidean division of `self` by `rhs`.
    #[must_use]
    fn div_euclid(self, rhs: Self) -> Self;

    /// Calculates the least nonnegative remainder of `self (mod rhs)`.
    #[must_use]
    fn rem_euclid(self, rhs: Self) -> Self;

    /// Calculates the middle point of `self` and `rhs`.
    #[must_use]
    fn midpoint(self, rhs: Self) -> Self;

    /// Returns the integer square root of a number.
    #[must_use]
    fn isqrt(self) -> Self;

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

macro_rules! impl_integer {
    ($($t:ty )*) => {
        $(
            impl Int for $t {
                const BITS: u32 = Self::BITS;

                #[inline(always)]
                fn count_ones(self) -> u32 {
                    Self::count_ones(self)
                }

                #[inline(always)]
                fn count_zeros(self) -> u32 {
                    Self::count_zeros(self)
                }

                #[inline(always)]
                fn leading_zeros(self) -> u32 {
                    Self::leading_zeros(self)
                }

                #[inline(always)]
                fn trailing_zeros(self) -> u32 {
                    Self::trailing_zeros(self)
                }

                #[inline(always)]
                fn leading_ones(self) -> u32 {
                    Self::leading_ones(self)
                }

                #[inline(always)]
                fn trailing_ones(self) -> u32 {
                    Self::trailing_ones(self)
                }

                #[inline(always)]
                fn rotate_left(self, n: u32) -> Self {
                    Self::rotate_left(self, n)
                }

                #[inline(always)]
                fn rotate_right(self, n: u32) -> Self {
                    Self::rotate_right(self, n)
                }

                #[inline(always)]
                fn swap_bytes(self) -> Self {
                    Self::swap_bytes(self)
                }

                #[inline(always)]
                fn reverse_bits(self) -> Self {
                    Self::reverse_bits(self)
                }

                #[inline(always)]
                fn from_be(x: Self) -> Self {
                    Self::from_be(x)
                }

                #[inline(always)]
                fn from_le(x: Self) -> Self {
                    Self::from_le(x)
                }

                #[inline(always)]
                fn to_be(self) -> Self {
                    Self::to_be(self)
                }

                #[inline(always)]
                fn to_le(self) -> Self {
                    Self::to_le(self)
                }

                #[inline(always)]
                fn checked_add(self, rhs: Self) -> Option<Self> {
                    Self::checked_add(self, rhs)
                }

                #[inline(always)]
                fn checked_sub(self, rhs: Self) -> Option<Self> {
                    Self::checked_sub(self, rhs)
                }

                #[inline(always)]
                fn checked_mul(self, rhs: Self) -> Option<Self> {
                    Self::checked_mul(self, rhs)
                }

                #[inline(always)]
                fn checked_div(self, rhs: Self) -> Option<Self> {
                    Self::checked_div(self, rhs)
                }

                #[inline(always)]
                fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
                    Self::checked_div_euclid(self, rhs)
                }

                #[inline(always)]
                fn checked_rem(self, rhs: Self) -> Option<Self> {
                    Self::checked_rem(self, rhs)
                }

                #[inline(always)]
                fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
                    Self::checked_rem_euclid(self, rhs)
                }

                #[inline(always)]
                fn checked_shl(self, rhs: u32) -> Option<Self> {
                    Self::checked_shl(self, rhs)
                }

                #[inline(always)]
                fn checked_shr(self, rhs: u32) -> Option<Self> {
                    Self::checked_shr(self, rhs)
                }

                #[inline(always)]
                fn checked_pow(self, exp: u32) -> Option<Self> {
                    Self::checked_pow(self, exp)
                }

                #[inline(always)]
                fn saturating_add(self, rhs: Self) -> Self {
                    Self::saturating_add(self, rhs)
                }

                #[inline(always)]
                fn saturating_sub(self, rhs: Self) -> Self {
                    Self::saturating_sub(self, rhs)
                }

                #[inline(always)]
                fn saturating_mul(self, rhs: Self) -> Self {
                    Self::saturating_mul(self, rhs)
                }

                #[inline(always)]
                fn saturating_div(self, rhs: Self) -> Self {
                    Self::saturating_div(self, rhs)
                }

                #[inline(always)]
                fn saturating_pow(self, exp: u32) -> Self {
                    Self::saturating_pow(self, exp)
                }

                #[inline(always)]
                fn wrapping_add(self, rhs: Self) -> Self {
                    Self::wrapping_add(self, rhs)
                }

                #[inline(always)]
                fn wrapping_sub(self, rhs: Self) -> Self {
                    Self::wrapping_sub(self, rhs)
                }

                #[inline(always)]
                fn wrapping_mul(self, rhs: Self) -> Self {
                    Self::wrapping_mul(self, rhs)
                }

                #[inline(always)]
                fn wrapping_div(self, rhs: Self) -> Self {
                    Self::wrapping_div(self, rhs)
                }

                #[inline(always)]
                fn wrapping_div_euclid(self, rhs: Self) -> Self {
                    Self::wrapping_div_euclid(self, rhs)
                }

                #[inline(always)]
                fn wrapping_rem(self, rhs: Self) -> Self {
                    Self::wrapping_rem(self, rhs)
                }

                #[inline(always)]
                fn wrapping_rem_euclid(self, rhs: Self) -> Self {
                    Self::wrapping_rem_euclid(self, rhs)
                }

                #[inline(always)]
                fn wrapping_shl(self, rhs: u32) -> Self {
                    Self::wrapping_shl(self, rhs)
                }

                #[inline(always)]
                fn wrapping_shr(self, rhs: u32) -> Self {
                    Self::wrapping_shr(self, rhs)
                }

                #[inline(always)]
                fn wrapping_pow(self, exp: u32) -> Self {
                    Self::wrapping_pow(self, exp)
                }

                #[inline(always)]
                fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                    Self::overflowing_add(self, rhs)
                }

                #[inline(always)]
                fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
                    Self::overflowing_sub(self, rhs)
                }

                #[inline(always)]
                fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
                    Self::overflowing_mul(self, rhs)
                }

                #[inline(always)]
                fn overflowing_div(self, rhs: Self) -> (Self, bool) {
                    Self::overflowing_div(self, rhs)
                }

                #[inline(always)]
                fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
                    Self::overflowing_div_euclid(self, rhs)
                }

                #[inline(always)]
                fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
                    Self::overflowing_rem(self, rhs)
                }

                #[inline(always)]
                fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
                    Self::overflowing_rem_euclid(self, rhs)
                }

                #[inline(always)]
                fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
                    Self::overflowing_shl(self, rhs)
                }

                #[inline(always)]
                fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
                    Self::overflowing_shr(self, rhs)
                }

                #[inline(always)]
                fn overflowing_pow(self, exp: u32) -> (Self, bool) {
                    Self::overflowing_pow(self, exp)
                }

                #[inline(always)]
                fn pow(self, exp: u32) -> Self {
                    Self::pow(self, exp)
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
                fn midpoint(self, rhs: Self) -> Self {
                    Self::midpoint(self, rhs)
                }

                #[inline(always)]
                fn isqrt(self) -> Self {
                    self.isqrt()
                }

                #[inline(always)]
                fn ilog(self, base: Self) -> u32 {
                    self.ilog(base)
                }

                #[inline(always)]
                fn ilog2(self) -> u32 {
                    self.ilog2()
                }

                #[inline(always)]
                fn ilog10(self) -> u32 {
                    self.ilog10()
                }

                #[inline(always)]
                fn checked_ilog(self, base: Self) -> Option<u32> {
                    self.checked_ilog(base)
                }

                #[inline(always)]
                fn checked_ilog2(self) -> Option<u32> {
                    self.checked_ilog2()
                }

                #[inline(always)]
                fn checked_ilog10(self) -> Option<u32> {
                    self.checked_ilog10()
                }
            }
        )*
    };
}

impl_integer!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
