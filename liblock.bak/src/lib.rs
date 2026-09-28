//! LibLock provides synchronization primitives that are absent in both `core` and `alloc`

#![no_std]

mod mutex;
pub use mutex::*;


pub mod helpers;


mod once;
pub use once::*;


mod rwlock;
pub use rwlock::*;


mod lazylock;
pub use lazylock::*;
