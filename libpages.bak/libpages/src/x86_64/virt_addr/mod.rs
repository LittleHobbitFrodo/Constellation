
mod indexes;
use core::{fmt::Pointer, ptr::NonNull};

#[cfg(test)]
mod tests;

pub use indexes::*;
use libmem::{Address, AlignedAddress, NonNullAddress, extent_alloc::extent::TakenExtent::Used};
//use libmem::{Address, PAGE_SIZE, PAGE_TABLE_ENTRY_COUNT};

use crate::{levels::TableIndexer, virt_addr::Pml4Indexer::Kernel};


/// Abstraction over a virtual address. This structure makes
/// it easier to use and manipulate page table indexes and
/// the virtual addresses in general
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct VirtualAddress(u64);

impl VirtualAddress {

    /// Constructs the `VirtualAddress` from the raw parts
    pub const fn from_parts(kernel: bool, pml4: u16, pdpt: u16, pd: u16, pt: u16, offset: u16) -> Self {
        let inner = (offset as u64 & Self::OFFSET_MASK)
            | ((pt as u64 & Self::NINE_BIT_MASK) << Self::PT_SHIFT)
            | ((pd as u64 & Self::NINE_BIT_MASK) << Self::PD_SHIFT)
            | ((pdpt as u64 & Self::NINE_BIT_MASK) << Self::PDPT_SHIFT)
            | ((pml4 as u64 & Self::NINE_BIT_MASK) << Self::PML4_SHIFT)
            | if kernel { Self::HIGH_BITS } else { 0 };

        Self(inner)
    }

    /// Indicates whether the address is null
    #[inline(always)]
    pub fn is_null(&self) -> bool { self.0 == 0 }

    /// Converts the `VirtualAddress` into a pointer
    #[inline(always)]
    pub fn as_ptr(&self) -> *const u8 { self.0 as _ }

    /// Converts the `VirtualAddress` into a non-null pointer
    #[inline(always)]
    pub fn as_non_null(&self) -> Option<NonNull<u8>> {
        unsafe {
            core::mem::transmute_copy(&self.0)
        }
    }


    /// Mask for each level
    const NINE_BIT_MASK: u64 = 0x1FF;

    /// Mask of the highest 16 bits that determines
    /// whether the address belongs to the kernel
    pub const HIGH_BITS: u64 = 0xFFFE << 48;

    /// Mask of the page offset
    pub const OFFSET_MASK: u64 = 0xFFF;

    /// Shift of the `PT` table index
    pub const PT_SHIFT: i32 = 12;

    /// Mask of the `PT` table index
    pub const PT_MASK: u64 = Self::NINE_BIT_MASK << Self::PT_SHIFT;

    /// Shift of the `PD` table index
    pub const PD_SHIFT: i32 = 21;

    /// Mask of the `PD` table index
    pub const PD_MASK: u64 = Self::NINE_BIT_MASK << Self::PD_SHIFT;

    /// Shift of the `PDPT` table index
    pub const PDPT_SHIFT: i32 = 30;

    /// Mask of the `PDPT` table entries index
    pub const PDPT_MASK: u64 = Self::NINE_BIT_MASK << Self::PDPT_SHIFT;

    /// Shift of the `PML4` table index
    pub const PML4_SHIFT: i32 = 39;

    /// Mask of the `PML4` table index
    pub const PML4_MASK: u64 = Self::NINE_BIT_MASK << Self::PML4_SHIFT;



    /// Returns the offset within the page
    #[inline(always)]
    pub fn offset(&self) -> u16 { (self.0 as u16) & 0xFFF }


    /// Sets the offset of this address\
    #[inline(always)]
    pub fn set_offset(&mut self, offset: u16) {
        self.0 = (self.0 & !Self::OFFSET_MASK) | (offset as u64 & Self::OFFSET_MASK)
    }


    /// Returns the index of the `PT` table
    #[inline(always)]
    pub fn pt_index(&self) -> PtIndexer {
        let idx = ((self.0 >> Self::PT_SHIFT) & Self::NINE_BIT_MASK) as u16;
        unsafe {
            PtIndexer::new_unchecked(idx)
        }
    }

    /// Sets the `PT` table index
    #[inline(always)]
    pub fn set_pt(&mut self, pt: u16) {
        self.0 = (self.0 & !Self::PT_MASK) | ((pt as u64 & Self::NINE_BIT_MASK) << Self::PT_SHIFT)
    }


    /// Returns the index of the `PD` table
    #[inline(always)]
    pub fn pd_index(&self) -> PdIndexer {
        let idx = ((self.0 >> Self::PD_SHIFT) & Self::NINE_BIT_MASK) as u16;
        unsafe {
            PdIndexer::new_unchecked(idx)
        }
    }

    /// Sets the `PD` table index
    #[inline(always)]
    pub fn set_pd(&mut self, pd: u16) {
        self.0 = (self.0 & !Self::PD_MASK) | ((pd as u64 & Self::NINE_BIT_MASK) << Self::PD_SHIFT)
    }

    /// Returns the index of the `PDPT` table
    #[inline(always)]
    pub fn pdpt_index(&self) -> PdPtIndexer {
        let idx = ((self.0 >> Self::PDPT_SHIFT) & Self::NINE_BIT_MASK) as u16;
        unsafe {
            PdPtIndexer::new_unchecked(idx)
        }
    }

    /// Sets the `PDPT` table index
    #[inline(always)]
    pub fn set_pdpt(&mut self, pdpt: u16) {
        self.0 = (self.0 & !Self::PDPT_MASK) | ((pdpt as u64 & Self::NINE_BIT_MASK) << Self::PDPT_SHIFT)
    }

    /// Returns the index of the `PML4` table
    #[inline(always)]
    pub fn pml4_index(&self) -> Pml4Indexer {
        let raw_idx = (self.0 >> Self::PML4_SHIFT) & Self::NINE_BIT_MASK;
        if raw_idx >= 256 {
            Pml4Indexer::Kernel((raw_idx >> 8) as u8)
        } else {
            Pml4Indexer::User(raw_idx as u8)
        }
    }

    /// Sets the `PML4` table index
    #[inline(always)]
    pub fn set_pml4(&mut self, pml4: u16) {
        self.0 = (self.0 & !Self::PML4_MASK) | ((pml4 as u64 & Self::NINE_BIT_MASK) << Self::PML4_SHIFT)
    }


    /// Checks the high bits of the address and returns
    /// whether the address belongs to the kernel
    #[inline(always)]
    pub fn is_kernel_address(&self) -> bool {
        self.0 & Self::HIGH_BITS == Self::HIGH_BITS
    }

    /// Checks the high bits of the address and returns
    /// whether the address belongs to userspace
    #[inline(always)]
    pub fn is_user_address(&self) -> bool {
        (self.0 >> 48) as u16 != u16::MAX
    }


    /// The address will belong to the kernel
    #[inline(always)]
    pub fn make_kernel(&mut self) { self.0 |= Self::HIGH_BITS }

    /// The address will belong to the user
    #[inline(always)]
    pub fn make_user(&mut self) { self.0 &= !Self::HIGH_BITS }
}


impl VirtualAddress {
    //  these functions are working only with 48bit addresses
    //    - The highest 16 bits must remain unchanged


    /// Adds the given offset/bytes to the virtual address
    /// - Keeps the highest 16 bits intact
    /// - This operation invokes `saturating_add` to prevent overflows
    pub fn add_offset(&mut self, bytes: usize) {
        let high = self.0 & Self::HIGH_BITS;

        let new = unsafe {
            self.0.unchecked_shl(16).saturating_add((bytes.unchecked_shl(16)) as u64).unchecked_shr(16)
        };
        self.0 = new | high;
    }


    /// Subtracts the given offset/bytes from the virtual address
    /// - Keeps the highest 16 bits intact
    /// - This operation invokes `saturating_sub` to prevent overflows
    pub fn sub_offset(&mut self, bytes: usize) {
        let high = self.0 & Self::HIGH_BITS;

        self.0 = (self.0 & !Self::HIGH_BITS).saturating_sub(bytes as u64) | high;
    }


    /// Adds the given `PT` entries to the virtual address
    /// - Keeps the highest 16 bits intact
    /// - This operation invokes `saturating_add` to prevent overflows
    pub fn add_pt(&mut self, entries: usize) {
        let high = self.0 & Self::HIGH_BITS;

        let new = unsafe {
            let entries = entries.unchecked_shl(16 + Self::PT_SHIFT as u32);
            self.0.unchecked_shl(16).saturating_add(entries as u64).unchecked_shr(16)
        };

        self.0 = new | high;
    }

    /// Subtracts the given `PT` entries from the virtual address
    /// - Keeps the highest 16 bits intact
    /// - This operation invokes `saturating_sub` to prevent overflows
    pub fn sub_pt(&mut self, entries: usize) {
        let high = self.0 & Self::HIGH_BITS;

        let entries = unsafe { entries.unchecked_shl(Self::PT_SHIFT as u32) };

        self.0 = (self.0 & !Self::HIGH_BITS).saturating_sub(entries as u64) | high;
    }






    /// Adds the given `PD` entries to the virtual address
    /// - Keeps the highest 16 bits intact
    /// - This operation invokes `saturating_add` to prevent overflows
    pub fn add_pd(&mut self, entries: usize) {
        let high = self.0 & Self::HIGH_BITS;

        let new = unsafe {
            let entries = entries.unchecked_shl(16 + Self::PD_SHIFT as u32);
            self.0.unchecked_shl(16).saturating_add(entries as u64).unchecked_shr(16)
        };

        self.0 = new | high;
    }


    /// Subtracts the given `PD` entries from the virtual address
    /// - Keeps the highest 16 bits intact
    /// - This operation invokes `saturating_sub` to prevent overflows
    pub fn sub_pd(&mut self, entries: usize) {
        let high = self.0 & Self::HIGH_BITS;

        let entries = unsafe { entries.unchecked_shl(Self::PD_SHIFT as u32) };

        self.0 = (self.0 & !Self::HIGH_BITS).saturating_sub(entries as u64) | high;
    }





    /// Adds the given `PDPT` entries to the virtual address
    /// - Keeps the highest 16 bits intact
    /// - This operation invokes `saturating_add` to prevent overflows
    pub fn add_pdpt(&mut self, entries: usize) {
        let high = self.0 & Self::HIGH_BITS;

        let new = unsafe {
            let entries = entries.unchecked_shl(16 + Self::PDPT_SHIFT as u32);
            self.0.unchecked_shl(16).saturating_add(entries as u64).unchecked_shr(16)
        };

        self.0 = new | high;
    }


    /// Subtracts the given `PDPT` entries from the virtual address
    /// - Keeps the highest 16 bits intact
    /// - This operation invokes `saturating_sub` to prevent overflows
    pub fn sub_pdpt(&mut self, entries: usize) {
        let high = self.0 & Self::HIGH_BITS;

        let entries = unsafe { entries.unchecked_shl(Self::PDPT_SHIFT as u32) };

        self.0 = (self.0 & !Self::HIGH_BITS).saturating_sub(entries as u64) | high;
    }



    /// Adds the given `PML4` entries to the virtual address
    /// - Keeps the highest 16 bits intact
    /// - This operation invokes `saturating_add` to prevent overflows
    pub fn add_pml4(&mut self, entries: usize) {
        let high = self.0 & Self::HIGH_BITS;

        let new = unsafe {
            let entries = entries.unchecked_shl(16 + Self::PML4_SHIFT as u32);
            self.0.unchecked_shl(16).saturating_add(entries as u64).unchecked_shr(16)
        };

        self.0 = new | high;
    }


    /// Subtracts the given `PML4` entries from the virtual address
    /// - Keeps the highest 16 bits intact
    /// - This operation invokes `saturating_sub` to prevent overflows
    pub fn sub_pml4(&mut self, entries: usize) {
        let high = self.0 & Self::HIGH_BITS;

        let entries = unsafe { entries.unchecked_shl(Self::PML4_SHIFT as u32) };

        self.0 = (self.0 & !Self::HIGH_BITS).saturating_sub(entries as u64) | high;
    }
}


impl<T> From<*const T> for VirtualAddress {
    #[inline(always)]
    fn from(value: *const T) -> Self { Self(value as u64) }
}

impl<T> From<*mut T> for VirtualAddress {
    #[inline(always)]
    fn from(value: *mut T) -> Self { Self(value as u64) }
}

impl<T> From<NonNull<T>> for VirtualAddress {
    #[inline(always)]
    fn from(value: NonNull<T>) -> Self { Self(value.as_ptr() as u64) }
}


impl core::fmt::Debug for VirtualAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "VirtualAddress(0x{:x}) {{ owner: {}, pml4: {}, pdpt: {}, pd: {}, pt: {}, off: {} }}",
            self.0,
            if self.is_kernel_address() { "kernel" } else { "user" },
            self.pml4_index().unwrap(),
            self.pdpt_index().as_u16(),
            self.pd_index().as_u16(),
            self.pt_index().as_u16(),
            self.offset()
        )
    }
}

impl core::fmt::Pointer for VirtualAddress {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Pointer::fmt(&(self.0 as *const u8), f)
    }
}


/// The PML4 table is split into two halves:
/// - The userspace (lower 256 entries)
/// - The kernelspace (higher 256 entries)
#[repr(u32)]
pub enum Pml4Indexer {
    User(u8),
    Kernel(u8),
}

impl Pml4Indexer {

    /// Converts the indexer into `u16`
    #[inline]
    pub fn into_u16(self) -> u16 {
        match self {
            Self::User(idx) => idx as u16,
            Self::Kernel(idx) => idx as u16 + u8::MAX as u16,
        }
    }

    /// Uwraps the indexer and returns the
    /// index of one of the two halves of the `PML4` table
    #[inline]
    pub fn unwrap(self) -> u8 {
        match self {
            Self::Kernel(idx) => idx,
            Self::User(idx) => idx,
        }
    }
}
