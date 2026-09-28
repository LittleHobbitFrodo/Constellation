//! TODO: TEST

use core::num::NonZero;
use libmem::extent_alloc::layout::LayoutDescriptor;

//#[cfg(target_arch = "x86_64")]
mod x86_64;

//#[cfg(target_arch = "aarch64")]
//mod arm64;


/// Used by the physical allocator to allocate contignous physical frames
/// - Automatically calculates alignment of the physical frame
///
/// `PhysicalLayout` is a wrapper around `NonZero<usize>`, so wrapping it in a `Option` is zero-cost
#[derive(Clone)]
#[repr(transparent)]
pub struct PhysicalLayout(pub(crate) NonZero<u64>);
    //  architecture-specific implementation
    //  - Needs to fit paging constraints defined by the MMU
    //
    //  Needs to implement the `LayoutDescriptor<PAGE_SIZE>` for each target

#[cfg(debug_assertions)]
impl core::fmt::Debug for PhysicalLayout {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "PhysicalLayout {{ count: {}, align: {} }}", self.size(), self.align())
    }
}
