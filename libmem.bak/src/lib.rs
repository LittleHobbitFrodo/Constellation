//! Provides memory management related subsystems for the Constelation kernel
//!
//! This includes:
//! - Extent allocator (work in progress)
//! - Global allocator (not yet implemented)
//! - And more!


#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]
#![deny(unsafe_op_in_unsafe_fn)]

#![no_std]

#[cfg(feature = "extent_alloc")]
pub mod extent_alloc;

mod misc;
pub use misc::*;

mod address;
pub use address::*;

//use crate::extent_alloc::allocator::ExtentAllocator;

extern crate alloc;


/// Same as the `panic!()` macro, but hints a `cold_path()`
/// to tell the compiler to optimize other paths
///
/// Panics in a kernel may not be unlikely (early development
/// stage, etc.), but given that a panic only occurs once in a
/// program lifetime (and terminates the panicking program),
/// calling `cold_panic!()` insead of `panic!()` may boost
/// kernel performance by a little
#[macro_export]
macro_rules! cold_panic {
    () => {{
        core::hint::cold_path();
        panic!();
    }};
    ($($arg:tt)*) => {{
        core::hint::cold_path();
        panic!($($arg)*);
    }};
}


/// Explicitly marks assumptions made by a programmer. This macro generally
/// expands into the `debug_asset!()` macro that asserts the invariant only
/// in debug mode (`#[cfg(debug_assertions)]`). However, there
/// is second, strict variant that uses the `assert!()` macro
/// that always asserts the invariant.
///
/// # Usage
/// The difference between `assume!()` and any other assertion is
/// that (by the name), `assume!()` marks implicit assumtions that
/// may not be clear to other programmers reading/writing code.
///
/// ```rust
/// fn do_something(input: u32) {
///     //  this function assumes that the input is always less than 32
///     //  - expands into `debug_assert!()`
///     assume!(input < 32);
///
///     //  ...
/// }
/// ```
///
/// ```rust
/// fn do_something(input: u32) {
///     //  this function assumes that the input is always less than 32
///     //  - expands into `assert!()`
///     assume!(strict: input < 32);
///
///     //  ...
/// }
/// ```
#[macro_export]
macro_rules! assume {
    (strict: $invariant:expr) => { assert!($invariant, stringify!(assumption failed: $invariant)) };
    (strict: $invariant:expr, $msg:literal) => { assert!($invariant, $msg) };

    ($invariant:expr) => { debug_assert!($invariant, stringify!(assumption failed: $invariant)) };
    ($invariant:expr, $msg:literal) => { debug_assert!($invariant, $msg) };
}
