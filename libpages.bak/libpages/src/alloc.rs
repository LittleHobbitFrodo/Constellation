use libmem::extent_alloc::{CachedExtentAllocator, extent::{Extent, RawExtent}};
use libmem::PAGE_SIZE;

use crate::physical::{PhysicalExtent, PhysicalLayout};




#[repr(transparent)]
pub struct PhysicalAllocator<Ext: Extent<PAGE_SIZE> + RawExtent<PAGE_SIZE> = PhysicalExtent<PAGE_SIZE>>
    (CachedExtentAllocator<PAGE_SIZE, Ext, PhysicalLayout>);
