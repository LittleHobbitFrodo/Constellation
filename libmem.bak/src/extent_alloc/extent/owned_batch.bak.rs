use core::mem::ManuallyDrop;

use alloc::vec::Vec;

use crate::extent_alloc::{ExtentAlloc, extent::{BuiltinExt, Extent, OwnedExtent, RawExtent}};
use super::ScopedExtent;


/// An collection of allocated extents that will get deallocated when `drop`ped
#[repr(C)]
pub struct Batch<'alloc, const ALIGN: usize, Ext: Extent<ALIGN> + RawExtent<ALIGN>, Alloc: ExtentAlloc<ALIGN>> {
    exts: Vec<ScopedExtent<'alloc, ALIGN, Ext, Alloc>>,
    alloc: &'alloc Alloc,
}


impl<'alloc, const ALIGN: usize, Ext: Extent<ALIGN> + RawExtent<ALIGN>, Alloc: ExtentAlloc<ALIGN>> Batch<'alloc, ALIGN, Ext, Alloc> {

    pub(crate) const fn new(exts: Vec<ScopedExtent<'alloc, ALIGN, Ext, Alloc>>, alloc: &'alloc Alloc) -> Self {
        Self { exts, alloc }
    }

    /// Returns the contained extents
    #[inline(always)]
    pub fn as_slice(&self) -> &[ScopedExtent<'alloc, ALIGN, Ext, Alloc>] {
        self.exts.as_slice()
    }

    /// Returns the reference to the allocator
    pub fn alloc(&self) -> &'alloc Alloc { self.alloc }

    /// Appends the extent into the `Batch`
    ///
    /// > NOTE: This function is allowed to make an `assert!`ion that
    /// the extent is allocated by the same allocator instance
    pub fn append(&mut self, ext: OwnedExtent<'alloc, ALIGN, Ext, Alloc>) {
        debug_assert!((self.alloc() as * const Alloc).addr() == (ext.alloc() as *const Alloc).addr());

        self.exts.push(ext.leak());
    }

    /// Transforms the `Batch` into a vector of `ScopedExtent`
    pub fn into_vec(self) -> Vec<ScopedExtent<'alloc, ALIGN, Ext, Alloc>> {
        let me = ManuallyDrop::new(self);

        let vec = unsafe {
            ((&me.exts) as *const Vec<ScopedExtent<'alloc, ALIGN, Ext, Alloc>>).read()
        };
        _ = me;

        vec
    }

    /// Same as `into_vec()`, but returns the allocator as well
    pub fn into_vec_with_alloc(self) -> (Vec<ScopedExtent<'alloc, ALIGN, Ext, Alloc>>, &'alloc Alloc) {
        let me = ManuallyDrop::new(self);

        let vec = unsafe {
            ((&me.exts) as *const Vec<ScopedExtent<'alloc, ALIGN, Ext, Alloc>>).read()
        };
        let alloc = me.alloc();

        _ = me;

        (vec, alloc)
    }

}
