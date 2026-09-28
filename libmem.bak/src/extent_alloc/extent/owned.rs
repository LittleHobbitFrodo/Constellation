use crate::AlignedNonNull;
use core::num::NonZero;

use crate::extent_alloc::ExtentAlloc;
use crate::extent_alloc::extent::{AbstractExtent, BuiltinExt, Extent, RawExtent, ScopedExtent};




/// An abstraction over a raw extent that will ensure that the extent
/// will not outlive its allocator and will be deallocated when `drop`ped
#[repr(C)]
pub struct OwnedExtent<'alloc, const ALIGN: usize, Ext: Extent<ALIGN> + RawExtent<ALIGN>, Alloc: ExtentAlloc<ALIGN>> {
    ext: Ext,
    alloc: &'alloc Alloc,
}

//  mark extent defined within libmem
impl<'alloc, const ALIGN: usize, Ext: Extent<ALIGN> + RawExtent<ALIGN>, Alloc: ExtentAlloc<ALIGN>>
BuiltinExt for OwnedExtent<'alloc, ALIGN, Ext, Alloc> {}

//  mark abstraction over raw extents
impl<'alloc, const ALIGN: usize, Ext: Extent<ALIGN> + RawExtent<ALIGN>, Alloc: ExtentAlloc<ALIGN>>
AbstractExtent<'alloc, ALIGN, Ext, Alloc> for OwnedExtent<'alloc, ALIGN, Ext, Alloc> {

    /*#[inline(always)]
    fn from_ext_with_alloc(ext: Ext, alloc: &'alloc Alloc) -> Self {
        Self { ext, alloc }
    }*/
}


impl<'alloc, const ALIGN: usize, Ext: Extent<ALIGN> + RawExtent<ALIGN>, Alloc: ExtentAlloc<ALIGN>>
core::ops::Deref for OwnedExtent<'alloc, ALIGN, Ext, Alloc> {
    type Target = Ext;
    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.ext }
}




impl<'alloc, const ALIGN: usize, Ext: Extent<ALIGN> + RawExtent<ALIGN>, Alloc: ExtentAlloc<ALIGN>>
Drop for OwnedExtent<'alloc, ALIGN, Ext, Alloc> {
    fn drop(&mut self) {
        todo!("deallocate the extent")
    }
}




impl<'alloc, const ALIGN: usize, Ext: Extent<ALIGN> + RawExtent<ALIGN>, Alloc: ExtentAlloc<ALIGN>> OwnedExtent<'alloc, ALIGN, Ext, Alloc> {

    /// Leaks the `OwnedExtent` and returns the
    /// innner raw extent wrapped in `ScopedExtent`
    pub fn leak(self) -> ScopedExtent<'alloc, ALIGN, Ext, Alloc> {
        todo!();
    }

    /// Returns the reference to the allocator
    pub fn alloc(&self) -> &'alloc Alloc { self.alloc }

    /// Constructs a new `OwnedExtent` from the given raw extent and the allocator
    ///
    /// # Safety
    /// It is up to the aller to guarantee that the extent was allocated by the given allocator
    pub unsafe fn from_extent_and_alloc(ext: Ext, alloc: &'alloc Alloc) -> Self {
        Self { ext, alloc }
        //Self::from_ext_with_alloc(ext, alloc)
    }
}



/*impl<'alloc, const ALIGN: usize, Ext: Extent<ALIGN> + RawExtent<ALIGN>, Alloc: ExtentAlloc<ALIGN>>
Extent<ALIGN> for OwnedExtent<'alloc, ALIGN, Ext, Alloc> {

    #[inline(always)]
    fn is_touching(&self, other: &'_ Self) -> bool {
        self.ext.is_touching(&other.ext)
    }

    #[inline(always)]
    fn address(&self) -> AlignedNonNull<NonZero<u64>, ALIGN> {
        self.ext.address()
    }

    #[inline(always)]
    fn end_address(&self) -> AlignedNonNull<NonZero<u64>, ALIGN> {
        self.ext.end_address()
    }

    #[inline(always)]
    fn size(&self) -> NonZero<u64> { self.ext.size() }

    #[inline(always)]
    fn fits_into(&self, other: &'_ Self) -> bool {
        self.ext.fits_into(&other.ext)
    }
}*/
