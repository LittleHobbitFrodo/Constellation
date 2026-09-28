use core::num::NonZero;

use libmem::{AlignedNonNull, extent_alloc::extent::{Extent, RawExtent}};


mod layout;
pub use layout::PhysicalLayout;



/// An extent allocated by the `PhysicalAllocator`
#[derive(Clone)]
#[repr(C)]
pub struct PhysicalExtent<const ALIGN: usize> {
    start: AlignedNonNull<NonZero<u64>, ALIGN>,
    size: NonZero<u64>
}


impl<const ALIGN: usize> Extent<ALIGN> for PhysicalExtent<ALIGN> {
    #[inline(always)]
    fn address(&self) -> AlignedNonNull<NonZero<u64>, ALIGN> {
        self.start
    }

    #[inline(always)]
    fn size(&self) -> NonZero<u64> { self.size }
}


impl<const ALIGN: usize> RawExtent<ALIGN> for PhysicalExtent<ALIGN> {
    #[inline(always)]
    fn new(address: AlignedNonNull<NonZero<u64>, ALIGN>, pages: NonZero<u64>) -> Self {
        Self { start: address, size: pages }
    }
}
