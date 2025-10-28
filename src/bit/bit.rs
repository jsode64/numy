use crate::impl_trait;

use super::{BitAssignOps, BitOps};

/// A pure binary type.
///
/// Exposes:
/// - Non-assigning bitwise operations
/// - Assigning bitwise operations
pub trait Bit: Copy + BitAssignOps + BitOps {}

impl_trait!(Bit: i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
