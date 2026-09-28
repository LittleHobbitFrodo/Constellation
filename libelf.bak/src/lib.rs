//! Library for unified loading of ELF files across Cronos
//!
//! > Note: Purposefully supports only elf64
//!
//!
//! Implementation base of [this document](https://uclibc.org/docs/elf-64-gen.pdf)

#![no_std]

use core::ptr::NonNull;

#[cfg(feature = "raw")]
pub mod raw;

#[cfg(not(feature = "raw"))]
mod raw;

pub mod api;
pub use api::Elf64;



/// Same as `reinterpret_slice()`, but this function offsets the slice before reinterpretting the bytes
#[inline]
pub(crate) fn reinterpret_slice_at<T: Sized>(slice: &[u8], offset: usize) -> Option<&T> {
    reinterpret_slice(slice.get(offset..)?)
}

/// Reinterprets bits of the given slice as instance of `T` and returns a reference to the instance of `T`
///
/// Or returns `None` if the slice is too small to hold `T` or if it is unaligned
pub(crate) fn reinterpret_slice<T: Sized>(slice: &[u8]) -> Option<&T> {
    if slice.len() >= size_of::<T>() {
        let ptr = unsafe {
            NonNull::new_unchecked(slice.as_ptr() as *mut T)
        };
        //if ptr.is_aligned() {
            Some(unsafe { ptr.as_ref() })
        /*} else {
            None
        }*/
    } else {
        None
    }
}

/// Reinterprets the slice as an slice of `T` with `len` elements
#[inline]
pub(crate) fn reinterpret_slice_as_slice_at<T: Sized>(slice: &[u8], len: usize, offset: usize) -> Option<&[T]> {
    reinterprest_slice_as_slice(slice.get(offset..)?, len)
}

/// Reinterprests the slice as an slice of `T` with `len` elements
/// - Returns `None` if the slice is too small to hold the reinterpretted slice or if it is unaligned
pub(crate) fn reinterprest_slice_as_slice<T: Sized>(slice: &[u8], len: usize) -> Option<&[T]> {
    if slice.len() >= size_of::<T>().saturating_mul(len) {
        let ptr = unsafe {
            NonNull::new_unchecked(slice.as_ptr() as *mut T)
        };

        //if ptr.is_aligned() {
            Some(unsafe { NonNull::slice_from_raw_parts(ptr, len).as_ref() })
        /*} else {
            None
        }*/
    } else {
        None
    }
}
