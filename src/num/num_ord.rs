/// General ordering functionality.
///
/// For floating point exceptions, see [`f32`] or [`f64`]'s implementations for
/// the same functions.
///
/// [`f32`]: https://doc.rust-lang.org/std/primitive.f32.html
/// [`f64`]: https://doc.rust-lang.org/std/primitive.f64.html
pub trait NumOrd: PartialOrd {
    #[must_use]
    fn min(self, other: Self) -> Self;

    #[must_use]
    fn max(self, other: Self) -> Self;

    #[must_use]
    fn clamp(self, min: Self, max: Self) -> Self;
}

macro_rules! impl_num_ord_iu {
    ($($t:ty )*) => {
        $(
            impl NumOrd for $t {
                #[inline(always)]
                fn min(self, other: Self) -> Self {
                    <Self as Ord>::min(self, other)
                }

                #[inline(always)]
                fn max(self, other: Self) -> Self {
                    <Self as Ord>::max(self, other)
                }

                #[inline(always)]
                fn clamp(self, min: Self, max: Self) -> Self {
                    <Self as Ord>::clamp(self, min, max)
                }
            }
        )*
    };
}

impl_num_ord_iu!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

macro_rules! impl_num_ord_f {
    ($($t:ty )*) => {
        $(
            impl NumOrd for $t {
                #[inline(always)]
                fn min(self, other: Self) -> Self {
                    self.min(other)
                }

                #[inline(always)]
                fn max(self, other: Self) -> Self {
                    self.max(other)
                }

                #[inline(always)]
                fn clamp(self, min: Self, max: Self) -> Self {
                    self.clamp(min, max)
                }
            }
        )*
    };
}

impl_num_ord_f!(f32 f64);
