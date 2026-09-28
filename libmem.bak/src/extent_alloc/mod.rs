//! Provides various extent allocator types and all sorts of things to use them
//!
//! Allocators:
//! - `ExtentAllocator`: An implementation of an extent allocator that operates
//! with a predetermined page size, various sizes of allocated blocks, and any number of memory blocks
//! - `CachedExtentAllocator`: Same as `ExtentAllocator`, but this has another layer of cache built-in
//! - `PhysicalAllocator`: Architecture-specific extent allocator used to manage physical memory


pub mod layout;

pub mod extent;

pub(crate) mod helpers;

pub mod cache;


pub mod raw_alloc;


mod alloc;
pub use alloc::*;
mod cached_alloc;
pub use cached_alloc::*;


/// Internal trait marking all extent allocators
pub(crate) trait ExtentAllocMarker {}

/// Trait unifying all extent allocator
/// types to use in generics
#[allow(private_bounds)]
pub trait ExtentAlloc<const ALIGN: usize>
where Self: ExtentAllocMarker {

    //pub fn alloc(&self) -> Option<

}
