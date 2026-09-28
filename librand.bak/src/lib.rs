//! Provides various random number generators
#![no_std]

use core::sync::atomic::AtomicUsize;


pub mod test;


extern crate alloc;


//#[cfg(feature = "testing")]
pub(crate) static SEXTANT_SEED: AtomicUsize = AtomicUsize::new(0);


//#[cfg(feature = "sextant")]
unsafe extern "Rust" {
    /// Seed generator
    pub(crate) fn __sextant_randomization_hook(seed: &'_ AtomicUsize);
}
