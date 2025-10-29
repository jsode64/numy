mod bit;
mod float;
mod int;
mod num;

pub use bit::{Bit, BitAssignOps, BitOps};
pub use float::{Float, FloatConst};
pub use int::{Int, SignedInt, UnsignedInt};
pub use num::{Num, NumAssignOps, NumOps, NumOrd};

mod min_max;
mod signed;

pub use min_max::MinMax;
pub use signed::Signed;

macro_rules! impl_trait {
    ($trait:ident: $($t:ty)*) => {
        $(
            impl $trait for $t {}
        )*
    };
}

pub(crate) use impl_trait;
