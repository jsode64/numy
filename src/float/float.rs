use std::{cmp::Ordering, num::FpCategory};

use crate::{FloatConst, Num, Signed, UnsignedInt};

/// Flating point types.
///
/// Exposes:
/// - The `Num` trait
/// - Floating point constants
/// - Floating point functionality
///
/// See [`f32`] or [`f64`].
///
/// [`f32`]: https://doc.rust-lang.org/std/primitive.f32.html
/// [`f64`]: https://doc.rust-lang.org/std/primitive.f64.html
pub trait Float: Num + FloatConst + Signed {
    /// The unsigned integer variant of the type with the same size.
    ///
    /// This is not normally defined for floats and is only here to be the
    /// return type of `.to_bits` and input for `.from_bits`.
    type Bits: UnsignedInt;

    /// The byte array variant of the type with the same size.
    ///
    /// This is not normally defined for floats and is only here for the
    /// to/from endian functons.
    type Bytes;

    /// The radix or base of the internal representation.
    const RADIX: u32;

    /// Number of significant digits in base 2.
    const MANTISSA_DIGITS: u32;

    /// Approximate number of significant digits in base 10.
    const DIGITS: u32;

    /// Machine epsilon value for this floating point type.
    const EPSILON: Self;

    /// Smallest positive normal value.
    const MIN_POSITIVE: Self;

    /// One greater than the minimum possible normal power of 2 exponent.
    const MIN_EXP: i32;

    /// One greater than the maximum possible power of 2 exponent.
    const MAX_EXP: i32;

    /// Minimum x for which 10^x is normal.
    const MIN_10_EXP: i32;

    /// Maximum x for which 10^x is normal.
    const MAX_10_EXP: i32;

    /// Not a Number (NaN).
    const NAN: Self;

    /// Infinity (∞).
    const INFINITY: Self;

    /// Negative infinity (−∞).
    const NEG_INFINITY: Self;

    /// Returns the largest integer less than or equal to `self`.
    #[must_use]
    fn floor(self) -> Self;

    /// Returns the smallest integer greater than or equal to `self`.
    #[must_use]
    fn ceil(self) -> Self;

    /// Returns the nearest integer to `self`.
    #[must_use]
    fn round(self) -> Self;

    /// Returns the nearest integer to a number.
    #[must_use]
    fn round_ties_even(self) -> Self;

    /// Returns the integer part of `self`.
    #[must_use]
    fn trunc(self) -> Self;

    /// Returns the fractional part of `self`.
    #[must_use]
    fn fract(self) -> Self;

    /// Fused multiply-add.
    #[must_use]
    fn mul_add(self, a: Self, b: Self) -> Self;

    /// Calculates Euclidean division, the matching method for `rem_euclid`.
    #[must_use]
    fn div_euclid(self, rhs: Self) -> Self;

    /// Calculates the least nonnegative remainder of `self (mod rhs)`.
    #[must_use]
    fn rem_euclid(self, rhs: Self) -> Self;

    /// Raises a number to an integer power.
    #[must_use]
    fn powi(self, n: i32) -> Self;

    /// Raises a number to a floating point power.
    #[must_use]
    fn powf(self, n: Self) -> Self;

    /// Returns the square root of a number.
    #[must_use]
    fn sqrt(self) -> Self;

    /// Returns `e^(self)`.
    #[must_use]
    fn exp(self) -> Self;

    /// Returns `2^(self)`.
    #[must_use]
    fn exp2(self) -> Self;

    /// Returns the natural logarithm of a number.
    #[must_use]
    fn ln(self) -> Self;

    /// Returns the logarithm of the number with respect to an arbitrary base.
    #[must_use]
    fn log(self, base: Self) -> Self;

    /// Returns the base 2 logarithm of the number.
    #[must_use]
    fn log2(self) -> Self;

    /// Returns the base 10 logarithm of the number.
    #[must_use]
    fn log10(self) -> Self;

    /// Returns the cube root of a number.
    #[must_use]
    fn cbrt(self) -> Self;

    /// Compute the distance between the origin and a point (`x`, `y`) on the Euclidean plane.
    #[must_use]
    fn hypot(self, other: Self) -> Self;

    /// Computes the sine of a number (in radians).
    #[must_use]
    fn sin(self) -> Self;

    /// Computes the cosine of a number (in radians).
    #[must_use]
    fn cos(self) -> Self;

    /// Computes the tangent of a number (in radians).
    #[must_use]
    fn tan(self) -> Self;

    /// Computes the arcsine of a number.
    #[must_use]
    fn asin(self) -> Self;

    /// Computes the arccosine of a number.
    #[must_use]
    fn acos(self) -> Self;

    /// Computes the arctangent of a number.
    #[must_use]
    fn atan(self) -> Self;

    /// Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`).
    #[must_use]
    fn atan2(self, other: Self) -> Self;

    /// Simultaneously computes the sine and cosine of the number, `x`.
    #[must_use]
    fn sin_cos(self) -> (Self, Self);

    /// Returns `e^(self) - 1` in a way that is accurate even if the number is close to zero.
    #[must_use]
    fn exp_m1(self) -> Self;

    /// Returns `ln(1+n)` (natural logarithm) more accurately than if the operations were performed separately.
    #[must_use]
    fn ln_1p(self) -> Self;

    /// Hyperbolic sine function.
    #[must_use]
    fn sinh(self) -> Self;

    /// Hyperbolic cosine function.
    #[must_use]
    fn cosh(self) -> Self;

    /// Hyperbolic tangent function.
    #[must_use]
    fn tanh(self) -> Self;

    /// Inverse hyperbolic sine function.
    #[must_use]
    fn asinh(self) -> Self;

    /// Inverse hyperbolic cosine function.
    #[must_use]
    fn acosh(self) -> Self;

    /// Inverse hyperbolic tangent function.
    #[must_use]
    fn atanh(self) -> Self;

    /// Returns `true` if this value is NaN.
    #[must_use]
    fn is_nan(self) -> bool;

    /// Returns `true` if this value is positive or negative infinity.
    #[must_use]
    fn is_infinite(self) -> bool;

    /// Returns `true` if this number is neither infinite nor NaN.
    #[must_use]
    fn is_finite(self) -> bool;

    /// Returns `true` if the number is subnormal.
    #[must_use]
    fn is_subnormal(self) -> bool;

    /// Returns `true` if the number is neither zero, infinite, subnormal, or NaN.
    #[must_use]
    fn is_normal(self) -> bool;

    /// Returns the floating point category of the number.
    #[must_use]
    fn classify(self) -> FpCategory;

    /// Returns `true` if `self` has a positive sign.
    #[must_use]
    fn is_sign_positive(self) -> bool;

    /// Returns `true` if `self` has a negative sign.
    #[must_use]
    fn is_sign_negative(self) -> bool;

    /// Returns the least number greater than `self`.
    #[must_use]
    fn next_up(self) -> Self;

    /// Returns the greatest number less than `self`.
    #[must_use]
    fn next_down(self) -> Self;

    /// Takes the reciprocal (inverse) of a number, `1/x`.
    #[must_use]
    fn recip(self) -> Self;

    /// Converts radians to degrees.
    #[must_use]
    fn to_degrees(self) -> Self;

    /// Converts degrees to radians.
    #[must_use]
    fn to_radians(self) -> Self;

    /// Returns the maximum of the two numbers, ignoring NaN.
    #[must_use]
    fn max(self, other: Self) -> Self;

    /// Returns the minimum of the two numbers, ignoring NaN.
    #[must_use]
    fn min(self, other: Self) -> Self;

    /// Calculates the midpoint (average) between `self` and `rhs`.
    #[must_use]
    fn midpoint(self, other: Self) -> Self;

    /// Raw transmutation to `u32`.
    #[must_use]
    fn to_bits(self) -> Self::Bits;

    /// Raw transmutation from `u32`.
    #[must_use]
    fn from_bits(v: Self::Bits) -> Self;

    /// Returns the memory representation as a big-endian byte array.
    #[must_use]
    fn to_be_bytes(self) -> Self::Bytes;

    /// Returns the memory representation as a little-endian byte array.
    #[must_use]
    fn to_le_bytes(self) -> Self::Bytes;

    /// Returns the memory representation as a native-endian byte array.
    #[must_use]
    fn to_ne_bytes(self) -> Self::Bytes;

    /// Creates a float from a big-endian byte array.
    #[must_use]
    fn from_be_bytes(bytes: Self::Bytes) -> Self;

    /// Creates a float from a little-endian byte array.
    #[must_use]
    fn from_le_bytes(bytes: Self::Bytes) -> Self;

    /// Creates a float from a native-endian byte array.
    #[must_use]
    fn from_ne_bytes(bytes: Self::Bytes) -> Self;

    /// Returns the ordering between `self` and `other` according to total order.
    #[must_use]
    fn total_cmp(&self, other: &Self) -> Ordering;

    /// Restrict a value to a certain interval unless it is NaN.
    #[must_use]
    fn clamp(self, min: Self, max: Self) -> Self;

    /// Returns a number that represents the sign of `self`.
    #[must_use]
    fn signum(self) -> Self;

    /// Returns a number composed of the magnitude of `self` and the sign of `sign`.
    #[must_use]
    fn copysign(self, sign: Self) -> Self;
}

macro_rules! impl_float {
    ($($t:ty, $b:ty);* $(;)*) => {
        $(
            impl Float for $t {
                type Bits = $b;

                type Bytes = [u8; std::mem::size_of::<Self>()];

                const RADIX: u32 = Self::RADIX;

                const MANTISSA_DIGITS: u32 = Self::MANTISSA_DIGITS;

                const DIGITS: u32 = Self::DIGITS;

                const EPSILON: Self = Self::EPSILON;

                const MIN_POSITIVE: Self = Self::MIN_POSITIVE;

                const MIN_EXP: i32 = Self::MIN_EXP;

                const MAX_EXP: i32 = Self::MAX_EXP;

                const MIN_10_EXP: i32 = Self::MIN_10_EXP;

                const MAX_10_EXP: i32 = Self::MAX_10_EXP;

                const NAN: Self = Self::NAN;

                const INFINITY: Self = Self::INFINITY;

                const NEG_INFINITY: Self = Self::NEG_INFINITY;

                #[inline(always)]
                fn floor(self) -> Self { Self::floor(self) }

                #[inline(always)]
                fn ceil(self) -> Self { Self::ceil(self) }

                #[inline(always)]
                fn round(self) -> Self { Self::round(self) }

                #[inline(always)]
                fn round_ties_even(self) -> Self { Self::round_ties_even(self) }

                #[inline(always)]
                fn trunc(self) -> Self { Self::trunc(self) }

                #[inline(always)]
                fn fract(self) -> Self { Self::fract(self) }

                #[inline(always)]
                fn mul_add(self, a: Self, b: Self) -> Self { Self::mul_add(self, a, b) }

                #[inline(always)]
                fn div_euclid(self, rhs: Self) -> Self { Self::div_euclid(self, rhs) }

                #[inline(always)]
                fn rem_euclid(self, rhs: Self) -> Self { Self::rem_euclid(self, rhs) }

                #[inline(always)]
                fn powi(self, n: i32) -> Self { Self::powi(self, n) }

                #[inline(always)]
                fn powf(self, n: Self) -> Self { Self::powf(self, n) }

                #[inline(always)]
                fn sqrt(self) -> Self { Self::sqrt(self) }

                #[inline(always)]
                fn exp(self) -> Self { Self::exp(self) }

                #[inline(always)]
                fn exp2(self) -> Self { Self::exp2(self) }

                #[inline(always)]
                fn ln(self) -> Self { Self::ln(self) }

                #[inline(always)]
                fn log(self, base: Self) -> Self { Self::log(self, base) }

                #[inline(always)]
                fn log2(self) -> Self { Self::log2(self) }

                #[inline(always)]
                fn log10(self) -> Self { Self::log10(self) }

                #[inline(always)]
                fn cbrt(self) -> Self { Self::cbrt(self) }

                #[inline(always)]
                fn hypot(self, other: Self) -> Self { Self::hypot(self, other) }

                #[inline(always)]
                fn sin(self) -> Self { Self::sin(self) }

                #[inline(always)]
                fn cos(self) -> Self { Self::cos(self) }

                #[inline(always)]
                fn tan(self) -> Self { Self::tan(self) }

                #[inline(always)]
                fn asin(self) -> Self { Self::asin(self) }

                #[inline(always)]
                fn acos(self) -> Self { Self::acos(self) }

                #[inline(always)]
                fn atan(self) -> Self { Self::atan(self) }

                #[inline(always)]
                fn atan2(self, other: Self) -> Self { Self::atan2(self, other) }

                #[inline(always)]
                fn sin_cos(self) -> (Self, Self) { Self::sin_cos(self) }

                #[inline(always)]
                fn exp_m1(self) -> Self { Self::exp_m1(self) }

                #[inline(always)]
                fn ln_1p(self) -> Self { Self::ln_1p(self) }

                #[inline(always)]
                fn sinh(self) -> Self { Self::sinh(self) }

                #[inline(always)]
                fn cosh(self) -> Self { Self::cosh(self) }

                #[inline(always)]
                fn tanh(self) -> Self { Self::tanh(self) }

                #[inline(always)]
                fn asinh(self) -> Self { Self::asinh(self) }

                #[inline(always)]
                fn acosh(self) -> Self { Self::acosh(self) }

                #[inline(always)]
                fn atanh(self) -> Self { Self::atanh(self) }

                #[inline(always)]
                fn is_nan(self) -> bool { Self::is_nan(self) }

                #[inline(always)]
                fn is_infinite(self) -> bool { Self::is_infinite(self) }

                #[inline(always)]
                fn is_finite(self) -> bool { Self::is_finite(self) }

                #[inline(always)]
                fn is_subnormal(self) -> bool { Self::is_subnormal(self) }

                #[inline(always)]
                fn is_normal(self) -> bool { Self::is_normal(self) }

                #[inline(always)]
                fn classify(self) -> FpCategory { Self::classify(self) }

                #[inline(always)]
                fn is_sign_positive(self) -> bool { Self::is_sign_positive(self) }

                #[inline(always)]
                fn is_sign_negative(self) -> bool { Self::is_sign_negative(self) }

                #[inline(always)]
                fn next_up(self) -> Self { Self::next_up(self) }

                #[inline(always)]
                fn next_down(self) -> Self { Self::next_down(self) }

                #[inline(always)]
                fn recip(self) -> Self { Self::recip(self) }

                #[inline(always)]
                fn to_degrees(self) -> Self { Self::to_degrees(self) }

                #[inline(always)]
                fn to_radians(self) -> Self { Self::to_radians(self) }

                #[inline(always)]
                fn max(self, other: Self) -> Self { Self::max(self, other) }

                #[inline(always)]
                fn min(self, other: Self) -> Self { Self::min(self, other) }

                #[inline(always)]
                fn midpoint(self, other: Self) -> Self { Self::midpoint(self, other) }

                #[inline(always)]
                fn to_bits(self) -> Self::Bits { Self::to_bits(self) }

                #[inline(always)]
                fn from_bits(v: Self::Bits) -> Self { Self::from_bits(v) }

                #[inline(always)]
                fn to_be_bytes(self) -> Self::Bytes { Self::to_be_bytes(self) }

                #[inline(always)]
                fn to_le_bytes(self) -> Self::Bytes { Self::to_le_bytes(self) }

                #[inline(always)]
                fn to_ne_bytes(self) -> Self::Bytes { Self::to_ne_bytes(self) }

                #[inline(always)]
                fn from_be_bytes(bytes: Self::Bytes) -> Self { Self::from_be_bytes(bytes) }

                #[inline(always)]
                fn from_le_bytes(bytes: Self::Bytes) -> Self { Self::from_le_bytes(bytes) }

                #[inline(always)]
                fn from_ne_bytes(bytes: Self::Bytes) -> Self { Self::from_ne_bytes(bytes) }

                #[inline(always)]
                fn total_cmp(&self, other: &Self) -> Ordering { Self::total_cmp(self, other) }

                #[inline(always)]
                fn clamp(self, min: Self, max: Self) -> Self { Self::clamp(self, min, max) }

                #[inline(always)]
                fn signum(self) -> Self { Self::signum(self) }

                #[inline(always)]
                fn copysign(self, sign: Self) -> Self { Self::copysign(self, sign) }
            }
        )*
    };
}

impl_float!(
    f32, u32;
    f64, u64;
);
