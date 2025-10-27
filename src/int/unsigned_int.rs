use crate::{Int, SignedInt};

pub trait UnsignedInt: Int {
    /// The signed integer type with the same size.
    type S: SignedInt;

    /// Checked addition with a signed integer.
    #[must_use]
    fn checked_add_signed(self, rhs: Self::S) -> Option<Self>;

    /// Saturating addition with a signed integer.
    #[must_use]
    fn saturating_add_signed(self, rhs: Self::S) -> Self;

    /// Wrapping (modular) addition with a signed integer.
    #[must_use]
    fn wrapping_add_signed(self, rhs: Self::S) -> Self;

    /// Overflowing addition with a signed integer.
    #[must_use]
    fn overflowing_add_signed(self, rhs: Self::S) -> (Self, bool);

    /// Checked subtraction with a signed integer.
    #[must_use]
    fn checked_sub_signed(self, rhs: Self::S) -> Option<Self>;

    /// Saturating subtraction with a signed integer.
    #[must_use]
    fn saturating_sub_signed(self, rhs: Self::S) -> Self;

    /// Wrapping (modular) subtraction with a signed integer.
    #[must_use]
    fn wrapping_sub_signed(self, rhs: Self::S) -> Self;

    /// Overflowing subtraction with a signed integer.
    #[must_use]
    fn overflowing_sub_signed(self, rhs: Self::S) -> (Self, bool);

    /// Reinterpret bits as the signed integer of the same size.
    #[must_use]
    fn cast_signed(self) -> Self::S;
}

macro_rules! impl_unsigned_integer {
    ($($t:ty, $s:ty);* $(;)*) => {
        $(
            impl UnsignedInt for $t {
                type S = $s;

                #[inline(always)]
                fn checked_add_signed(self, rhs: Self::S) -> Option<Self> {
                    self.checked_add_signed(rhs)
                }

                #[inline(always)]
                fn saturating_add_signed(self, rhs: Self::S) -> Self {
                    self.saturating_add_signed(rhs)
                }

                #[inline(always)]
                fn wrapping_add_signed(self, rhs: Self::S) -> Self {
                    self.wrapping_add_signed(rhs)
                }

                #[inline(always)]
                fn overflowing_add_signed(self, rhs: Self::S) -> (Self, bool) {
                    self.overflowing_add_signed(rhs)
                }

                #[inline(always)]
                fn checked_sub_signed(self, rhs: Self::S) -> Option<Self> {
                    self.checked_sub_signed(rhs)
                }

                #[inline(always)]
                fn saturating_sub_signed(self, rhs: Self::S) -> Self {
                    self.saturating_sub_signed(rhs)
                }

                #[inline(always)]
                fn wrapping_sub_signed(self, rhs: Self::S) -> Self {
                    self.wrapping_sub_signed(rhs)
                }

                #[inline(always)]
                fn overflowing_sub_signed(self, rhs: Self::S) -> (Self, bool) {
                    self.overflowing_sub_signed(rhs)
                }

                #[inline(always)]
                fn cast_signed(self) -> Self::S {
                    self.cast_signed()
                }
            }
        )*
    };
}

impl_unsigned_integer!(
    u8, i8;
    u16, i16;
    u32, i32;
    u64, i64;
    u128, i128;
    usize, isize;
);
