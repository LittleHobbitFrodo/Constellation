use alloc::collections::BTreeMap;

use crate::{AlignedNonNull, cold_panic, extent_alloc::{
    cache, extent::{Extent, MutableExtent}, helpers::SizeMap, layout::{ExtentLayout, LayoutDescriptor}, raw_alloc::RawExtentAlloc,
}};
use core::{hint::cold_path, marker::PhantomData, num::NonZero, ops::Index};

use super::{ExtentCache, SourceAllocation, RefuelRequest};

/// Extent cache used by the `CachedExtentAllocator` by default
pub struct DefaultCache<const ALIGN: usize, Lay: LayoutDescriptor<ALIGN>> {
    cached: AlignmentCache<ALIGN>,
    _lay: PhantomData<Lay>
}

const ALIGN_ARRAY_LEN: usize = 4;

impl<const ALIGN: usize, Lay: LayoutDescriptor<ALIGN>> DefaultCache<ALIGN, Lay> {

    const PRE_ALLOC_EXT_SIZE: [NonZero<u64>; ALIGN_ARRAY_LEN] = unsafe {
        [
            NonZero::new_unchecked(512),    //  align: 1
            NonZero::new_unchecked(1024),   //  align: 2
            NonZero::new_unchecked(2048),   //  align: 4
            NonZero::new_unchecked(4096)    //  align: 8
        ]
    };

}

impl<const ALIGN: usize, Lay: LayoutDescriptor<ALIGN>> ExtentCache<ALIGN, Lay> for DefaultCache<ALIGN, Lay> {

    const NEW: Self = Self { cached: AlignmentCache::new(), _lay: PhantomData };

    fn allocate_cached(&mut self, layout: Lay) -> Result<Option<SourceAllocation<ALIGN>>, RefuelRequest> {

        let mut i = layout.align().trailing_zeros() as usize;

        let arr = self.cached.inner_mut();

        while let Some(map) = arr.get_mut(i) {



            if let Some(size_and_address) = map.range(layout.size()..).next() {

                //  tell the borrowchecker to shut
                let size = size_and_address.0.clone();
                let address = size_and_address.1.clone();
                let _ = size_and_address;

                //  remove the extent
                if let None = map.remove(&size) {
                    //  recovery: ignore the error and continue the loop
                    cold_path();

                    continue
                }

                let mut allocated = MutableExtent::new(address, size);

                if let Some(reinsert) = unsafe { allocated.split_unchecked(size) } {
                    todo!("reinsert the rest of the extent")
                }


                todo!();

            }

            i += 1;
        }

        Ok(None)
    }

    fn refuel(&mut self, gas_station: &mut RawExtentAlloc<ALIGN>) {
        let cached = self.cached.inner_mut();

        debug_assert!(cached.len() == Self::PRE_ALLOC_EXT_SIZE.len());

        for (i, map) in cached.iter_mut().enumerate() {
            if map.len() > 8 { continue; }

            let size = unsafe {
                //  safety: the constant's length is equal to the size of the `cached` array
                //  - this is asserted above
                Self::PRE_ALLOC_EXT_SIZE.get_unchecked(i).clone()
            };


            //  construct layout for the allocation
            let align = unsafe { NonZero::new_unchecked(1 << i) };

            let layout = unsafe {
                ExtentLayout::from_pages_with_align_unchecked(size, align)
            };


            //  allocate the extent
            if let Some(ext) = gas_station.allocate_exact(layout) {
                if let Some(_) = map.insert(ext.size(), ext.address()) {
                    //  recovery: deallocate the extent

                    cold_path();
                    if let Err(_) = gas_station.deallocate(MutableExtent::from_regular(ext)) {
                        //  this path should never be taken
                        cold_panic!("cache recovery failed: deallocation failed")
                    }
                }
            } else {
                //  ignored
                cold_path();
            }

        }
    }

}


type SizedMap<const ALIGN: usize> = BTreeMap<NonZero<u64>, AlignedNonNull<NonZero<u64>, ALIGN>>;

/// Holds an array of maps of extents. The array is indexed by the
/// `trailing_zeroes()` of the layout alignment and each map contains
/// `size` -> `address` pairs.
struct AlignmentCache<const ALIGN: usize>([SizedMap<ALIGN>; ALIGN_ARRAY_LEN]);

impl<const ALIGN: usize> AlignmentCache<ALIGN> {

    const fn new() -> Self {
        Self([SizedMap::new(), SizedMap::new(), SizedMap::new(), SizedMap::new()])
    }

    /// Returns a reference to the map that handles the given alignment
    #[inline(always)]
    pub fn get(&self, align: usize) -> Option<&SizedMap<ALIGN>> {
        self.0.get(align.trailing_zeros() as usize)
    }

    /// Returns a mutable reference to the map that handles the given alignment
    #[inline(always)]
    pub fn get_mut(&mut self, align: usize) -> Option<&mut SizedMap<ALIGN>> {
        self.0.get_mut(align.trailing_zeros() as usize)
    }


    /// Returns a reference to the inner array
    #[inline(always)]
    pub fn inner(&self) -> &[SizedMap<ALIGN>; 4] { &self.0 }

    /// Returns a mutable reference to the inner array
    #[inline(always)]
    pub fn inner_mut(&mut self) -> &mut [SizedMap<ALIGN>; 4] { &mut self.0 }

}
