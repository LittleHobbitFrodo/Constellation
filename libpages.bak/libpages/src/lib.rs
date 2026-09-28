#![no_std]


//  IDEA: quick-map pages are allocated per-AddressSpace
//    - if the address space is dropped, they are put
//      into a vector to prevent re-allocation

/// Contains either a regular page entry or page-sized page entry
pub enum Either<R: Sized, S: Sized> {
    Regular(R),
    Sized(S)
}

pub mod physical;


//#[cfg(any(target_arch = "x86_64", feature = "testing"))]
mod x86_64;

//#[cfg(any(target_arch = "x86_64", feature = "testing"))]
pub use x86_64::*;

mod alloc;


/*/// Marks a page entry of any kind (even non-atomic ones)
pub(crate) trait PageEntry where Self: Sized {}


/*/// Marker trait used to distinguish atomic page entry of any kind
pub(crate) trait AtomicPageEntry where Self: Sized {}*/


/// Marker trait used to distinguish atomic page entries that can be found in tables
/// - Eq. entries that can work as both page-sized and regular entries
pub(crate) trait PageEntryUnion where Self: PageEntry {}


/// A thread-safe page table
#[allow(private_bounds)]
pub struct PageTable<E: PageEntryUnion>([E; PAGE_TABLE_ELEMENT_COUNT]);

impl<E: PageEntryUnion> core::ops::Deref for PageTable<E> {
    type Target = [E; PAGE_TABLE_ELEMENT_COUNT];
    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<E: PageEntryUnion> core::ops::DerefMut for PageTable<E> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}*/



//  TODO: implement liblock
