mod bit;
mod float;
mod int;
mod num;

pub use bit::{Bit, BitAssignOps, BitOps};
pub use float::{Float, FloatConst};
pub use int::{Int, SignedInt, UnsignedInt};
pub use num::{Num, NumAssignOps, NumOps};

mod min_max;
mod signed;

pub use min_max::MinMax;
pub use signed::Signed;

#[cfg(feature = "util")]
mod util;

#[cfg(feature = "util")]
pub use util::NumUtil;

macro_rules! impl_trait {
    ($trait:ident: $($t:ty)*) => {
        $(
            impl $trait for $t {}
        )*
    };
}

pub(crate) use impl_trait;
