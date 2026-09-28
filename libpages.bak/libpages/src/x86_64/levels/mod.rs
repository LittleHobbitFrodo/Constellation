use core::{num::NonZero, ptr::NonNull};

use libmem::{AlignedAddress, AlignedNonNull, PAGE_SIZE, PAGE_TABLE_ENTRY_COUNT};

use crate::Either;


mod regular;
pub use regular::{Pml4Entry, PdptEntry, PdEntry};

mod union;


mod sized;




/// Marks all page entries
pub(crate) trait PageEntry
where Self: Sized + Copy + Clone {
    type TableIndex: TableIndexer;
}

/// Marks all sized page entr
pub(crate) trait SizedPageEntry
where Self: PageEntry {

    /// A bit mask of the physical address address
    const MASK_ADDRESS: u64;

    /// Dictates the alignment of the address
    const ADDRESS_ALIGN: usize;
}

/// Marks all union entries
pub(crate) trait PageEntryUnion
where Self: PageEntry {

    /// The regular version of this entry
    /// - The `PS` bit is cleared
    type Regular: PageEntry;

    /// The sized version of this entry
    /// - The `PS` is set
    type Sized: SizedPageEntry;

    /// Returns an reference to the underlying entry
    fn get_ref(&self) -> Either<&Self::Regular, &Self::Sized>;

    /// Returns an mutable reference to the underlying entry
    fn get_mut(&mut self) -> Either<&mut Self::Regular, &mut Self::Sized>;

    /// Returns the underlying entry
    fn get(&self) -> Either<Self::Regular, Self::Sized>;

}

/// Marks all page table indices. All marked indices
/// must hold one invariant: their value must be less
/// than the `PAGE_TABLE_ENTRY_COUNT` constant
pub(crate) trait TableIndexer
where Self: Sized + Clone {
    /// Constructs a new table index
    /// - Returns `None` if the `index` is greater than
    /// or equal to the `PAGE_TABLE_ENTRY` constant
    #[inline]
    fn new(index: u16) -> Option<Self> {
        if (index as usize) < PAGE_TABLE_ENTRY_COUNT {
            unsafe {
                Some(Self::new_unchecked(index))
            }
        } else {
            None
        }
    }


    /// Constructs a new table index without checking the number
    ///
    /// # Safety
    /// It is up to the caller to guarantee that the `index`
    /// is less than the `PAGE_TABLE_ENTRY_COUNT` constant.
    /// Breaking this rule will introduce undefined behaviour
    unsafe fn new_unchecked(index: u16) -> Self;

    /// Returns the index
    fn as_u16(&self) -> u16;
}
