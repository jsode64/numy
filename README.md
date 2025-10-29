# Numy

Trait boundaries for primitive Rust types.

# Features

Numy contains traits for selection of Rust primitive types, mainly useful for
generic guards.

For example: having a function take a type `T` that is only a numeric value, or
one that takes only floats, or one that takes only signed integers, etc.

To see the provided traits, see [the API's documentation](https://docs.rs/numy).

# Examples

```rust
use numy::{Float, Signed, UnsignedInt};

fn main() {
    foo(1.0_f64);
    bar(-1_i32);
    roy(7_u32);
}

/// Takes only floating point types (`f32` or `f64`).
fn foo<T: Float>(n: T) -> T {
    n.sqrt() * T::PI
}

/// Takes only signed types such as `i32` or `f64`.
fn bar<T: Signed>(n: T) -> T {
    n.abs() + T::ONE
}

/// Takes only unsigned integers such as `u32`.
fn roy<T: UnsignedInt>(n: T) -> T {
    // Note that `T::S` is the signed integer type with the same size as `T`.
    n.wrapping_add_signed(T::S::NEG_ONE)
}
```

`"util"` feature:

```rust
// Requires the `"util"` feature:
use numy::NumUtil;

fn main() {
    assert_eq!(1.abs_diff(3_i32), 2);
    assert_eq!(1.abs_diff(3_u32), 2);
    assert_eq!(1.0.abs_diff(3.0_f64), 2.0);
}

/// Returns the absolute difference of the two values.
fn abs_diff<T: NumUtil>(lhs: T, rhs: T) -> T {
    // `.diff` will never underflow for unsigned integers and `.abs` makes sure
    // signed types yield non-negative results.
    lhs.diff(rhs).abs()
}
```
