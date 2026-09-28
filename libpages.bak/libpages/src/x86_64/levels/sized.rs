

use core::num::NonZero;

use libmem::{AlignedNonNull, PAGE_SIZE, KB, MB, GB};

use crate::{levels::{PageEntry, SizedPageEntry}, virt_addr::{PdIndexer, PdPtIndexer, PtIndexer}};


/// Only bits that are used by all the sized entries in the same
/// way are implemented by this macro
/// - This includes the address
///
/// It is up to the caller to provide the `MASK_ADDRESS` constant
macro_rules! generate_sized_entry {
    ($entry_name:ident, $align:expr, $addr_mask:expr, $indexer:ident) => {

        /// Page entry that is pointing to a block of memory
        #[repr(transparent)]
        #[derive(Clone, Copy)]
        pub struct $entry_name(u64);

        impl PageEntry for $entry_name {
            type TableIndex = $indexer;
        }

        impl SizedPageEntry for $entry_name {

            const MASK_ADDRESS: u64 = $addr_mask;

            const ADDRESS_ALIGN: usize = $align;

        }


        impl $entry_name {

            /// Constructs the page entry from the given integer
            ///
            /// # Safety
            /// This function gives the caller absolute power over the page
            /// they are constructing. However, `libpages` holds certain
            /// rules and assumptions about its page tree structure.
            /// Violating any of there rules/assumptions may introduce
            /// undefined behaviour
            pub const unsafe fn from_u64(entry: u64) -> Self {
                Self(entry)
            }


            /// Constructs the page entry from the given integer
            ///
            /// # Safety
            /// This function gives the caller absolute power over the page
            /// they are constructing. However, `libpages` holds certain
            /// rules and assumptions about its page tree structure.
            /// Violating any of there rules/assumptions may introduce
            /// undefined behaviour
            pub const unsafe fn from_non_zero(entry: Option<NonZero<u64>>) -> Self {
                Self(unsafe { core::mem::transmute(entry) })
            }



            /// Constructs the default entry for kernel page trees
            /// - The address is set to NULL (use `with_address()` to set the physical address)
            ///
            /// # Result
            /// The returned entry is present, read-only and not executable
            /// - Also accessible only from kernelspace
            pub const fn new_kernel() -> Self {
                //  US is cleared
                Self(Self::BIT_PRESENT | Self::BIT_EXEC_DISABLE)
            }


            /// Constructs the default entry for userspace page trees
            /// - The address is set to NULL (use `with_address()` to set the physical address)
            ///
            /// # Result
            /// The returned entry is present, read-only and not executable
            /// - Also accessible from both userspace and kernelspace
            pub const fn new_userspace() -> Self {
                Self(Self::BIT_PRESENT | Self::BIT_RW | Self::BIT_US | Self::BIT_EXEC_DISABLE)
            }


            /// Constructs a new unused/NULL entry
            pub const fn new_unused() -> Self { Self(0) }

            /// Sets the address for a newly created entry
            #[inline(always)]
            pub fn with_address(self, addr: AlignedNonNull<NonZero<u64>, $align>) -> Self {
                Self(self.0 | (addr.get() & Self::MASK_ADDRESS))
            }

            /// Makes the newly create entry writeable
            pub const fn writeable(self) -> Self { Self(self.0 | Self::BIT_RW) }


            /// Bit 0: **P**RESENT
            /// - If set, the page is present and loaded in memory
            /// - If cleared, the page is swapped and the MMU will trigger page fault on access
            pub const BIT_PRESENT: u64 = 0b1;

            /// Bit 1: `RW` (**R**ead/**W**rite)
            /// - If set, the page is read + write
            /// - If cleared the page is read only
            ///
            /// The `WP` bit in the `cr0` register indicates whether write-protection is applied to userland
            ///   - The kernel has always write access by default
            ///
            /// The RW bit is also checked on parent tables
            pub const BIT_RW: u64 = 0b1 << 1;

            /// Bit 2: `US` (**U**ser/**S**upervisor)
            /// - If set, the page can be accessed by both kernel and userspace
            /// - If cleared, the page can be accessed by only the kernel
            pub const BIT_US: u64 = 0b1 << 2;

            /// Bit 3: `PWT`
            /// - If `PAT` is not supported or the `PAT` bit is cleared, must be cleared too
            ///
            /// If set, write-back caching is used for the page
            ///
            /// If cleared, write-through caching is used for the page
            ///
            /// Use the `PCD` bit to disable caching
            pub const BIT_PWT: u64 = 0b1 << 3;

            /// Bit 4: `PCD` (Cache-Disable)
            /// - If `PAT` is not supported or the `PAT` bit is cleared, `PCD` must be cleared too
            ///
            /// If set, caching will be disabled for this page
            ///
            /// If cleared, caching will be done accordingly to the state of the `PWT` bit
            pub const BIT_PCD: u64 = 0b1 << 4;

            /// Bit 5: `A`ccessed
            /// - Set by the CPU if this entry has been accessed by the
            /// MMU while translating virtual addresses to physical addresses
            pub const BIT_ACCESSED: u64 = 0b1 << 5;

            /// Bit 6: `D`irty
            ///
            /// Indicates whether a page has been written to
            pub const BIT_DIRTY: u64 = 0b1 << 6;

            //  Bit 7: PS is always 1

            /// Bit 8: `G`lobal
            ///
            /// Tells the CPU to not invalidate the TLB cache entry when performing `mov` to `cr3`
            /// - Bit 7 in `cr4` must be set to enable global pages
            pub const BIT_GLOBAL: u64 = 0b1 << 8;


            /// Bits 59..62: `P`rotection `K`ey
            ///
            /// PK, or 'Protection Key'. The protection key is a 4-bit corresponding to each virtual
            /// address that is used to control user-mode and supervisor-mode memory accesses. If
            /// the PKE bit (bit 22) in CR4 is set, then the PKRU register is used for determining
            /// access rights for user-mode based on the protection key. If the PKS bit (bit 24) is
            /// set in CR4, then the PKRS register is used for determining access rights for
            /// supervisor-mode based on the protection key. A protection key allows the system to
            /// enable/disable access rights for multiple page entries across different address spaces at once.
            pub const MASK_PROTECT_KEY: u64 = 0xF << 59;


            /// Bit 63: E**X**ecute **D**isable
            /// - If set, instructions are not allowed to be executed from this page
            ///
            /// If the `NXE` bit in the `EFER` is set, pages marked with `XD` cannot be executed
            /// - If it is cleared, the `XD` bit is reserved and should be cleared
            const BIT_EXEC_DISABLE: u64 = 0b1 << 63;



            /// Sets the bits that are set in the given `mask`
            #[inline(always)]
            fn set_bits(&mut self, mask: u64) { *self = Self(self.0 | mask) }

            /// Clears the bits that are set in the given `mask`
            #[inline(always)]
            fn clear_bits(&mut self, mask: u64) { *self = Self(self.0 & !mask) }






            /// Indicates whether the entry is NULL (and thus unused)
            #[inline(always)]
            pub fn is_unused(&self) -> bool { self.0 == 0 }




            /// Indicates whether the page is located in the physical memory at the moment
            #[inline(always)]
            pub fn is_present(&self) -> bool {
                self.0 & Self::BIT_PRESENT != 0
            }

            /// Sets the present bit
            #[inline(always)]
            pub fn make_present(&mut self) {
                self.set_bits(Self::BIT_PRESENT)
            }

            /// Clears the present bit
            #[inline(always)]
            pub fn make_unpresent(&mut self) {
                self.set_bits(Self::BIT_PRESENT)
            }



            /// Indicates whether the page table the entry
            /// is pointing to can be written into
            #[inline(always)]
            pub fn is_writeable(&self) -> bool { self.0 & Self::BIT_RW != 0 }

            /// Makes the page table the entry is pointing to writeable
            #[inline(always)]
            pub fn make_writeable(&mut self) { self.set_bits(Self::BIT_RW) }

            /// Makes the page table the entry is pointing to read only
            #[inline(always)]
            pub fn make_read_only(&mut self) { self.clear_bits(Self::BIT_RW); }




            /// Indicates whether the page table the entry is pointing to belongs to userspcae
            #[inline(always)]
            pub fn is_userspace(&self) -> bool {
                self.0 & Self::BIT_US != 0
            }

            /// Indicates whether the page table the entry is pointing to belongs to the kernel
            #[inline(always)]
            pub fn is_kernelspace(&self) -> bool {
                self.0 & Self::BIT_US == 0
            }

            /// Makes the table the entry is pointing to inaccessible by the userspace
            #[inline(always)]
            pub unsafe fn make_kernelspace(&mut self) {
                self.clear_bits(Self::BIT_US);
            }

            /// Makes the table the entry is pointing to part of the userspace
            #[inline(always)]
            pub unsafe fn make_userspace(&mut self) {
                self.set_bits(Self::BIT_US);
            }






            /// Indicates whether the page is cached by a write-though mechanism
            #[inline(always)]
            pub fn is_cached_write_through(&self) -> bool {
                self.0 & Self::BIT_PWT == 0
            }

            /// Indicates whether the page is cached by a write-back mechanism
            #[inline(always)]
            pub fn is_cached_write_back(&self) -> bool {
                self.0 & Self::BIT_PWT != 0
            }

            /// Uses the write-back cache mechanism
            #[inline(always)]
            pub fn cache_write_back(&mut self) {
                self.set_bits(Self::BIT_PWT);
            }

            /// Uses the write-though cache mechanism
            #[inline(always)]
            pub fn cache_write_through(&mut self) {
                self.clear_bits(Self::BIT_PWT);
            }




            /// Indicates wthether the page table the entry is pointing to is cached
            #[inline(always)]
            pub fn is_cached(&self) -> bool { self.0 & Self::BIT_PCD == 0 }

            /// Enables caching for the page table the entry is pointing to
            #[inline(always)]
            pub fn cache_enable(&mut self) { self.clear_bits(Self::BIT_PCD); }

            /// Disables caching for the page table the entry is pointing to
            ///
            /// > **SIDE EFFECT**: Due to x86_64 paging requirements, the `PWT` bit will be cleared
            #[inline(always)]
            pub fn cache_disable(&mut self) {
                *self = Self((self.0 | Self::BIT_PCD) & !Self::BIT_PWT);
                /*self.set_bits(Self::BIT_PCD);
                self.clear_bits(Self::BIT_PWT);*/
            }


            /// Shows the **A**ccessed bit which is set by the
            /// MMU while translating virtual addresses
            #[inline(always)]
            pub fn is_accessed(&self) -> bool {
                self.0 & Self::BIT_ACCESSED != 0
            }

            /// Clears the `ACCESSED` bit
            #[inline(always)]
            pub fn make_unaccessed(&mut self) {
                self.clear_bits(Self::BIT_ACCESSED);
            }


            /// Sets the `ACCESSED` bit
            #[inline(always)]
            pub fn make_accessed(&mut self) {
                self.set_bits(Self::BIT_ACCESSED);
            }



            /// Returns the raw address of the physical frame
            #[inline(always)]
            pub fn address(&self) -> Option<AlignedNonNull<NonZero<u64>, $align>> {
                let addr = self.0 & Self::MASK_ADDRESS;
                unsafe {
                    core::mem::transmute(addr)
                }
            }
        }

    };
}




generate_sized_entry!(PdPtSizedEntry, GB, 0xFFFFC0000000, PdPtIndexer);

/*impl PageEntry for PdPtSizedEntry {
    type TableIndex = PdPtIndexer;
}

impl PdPtSizedEntry {
    pub const MASK_ADDRESS: u64 = 0xFFFFC0000000;
    pub const FRAME_SIZE: u64 = GB as u64;
}*/





//const MB_2: usize = 2*MB;
generate_sized_entry!(PdSizedEntry, {2*MB}, 0xFFFFFFE00000, PdIndexer);

/*impl PageEntry for PdSizedEntry {
    type TableIndex = PdIndexer;
}
impl PdSizedEntry {
    pub const MASK_ADDRESS: u64 = 0xFFFFFFE00000;
    pub const FRAME_SIZE: u64 = 0x200000;
}*/





//const KB_4: usize = 4096;
generate_sized_entry!(PtEntry, 4096, 0xFFFFFFFFF000, PtIndexer);
/*impl PtEntry {
    pub const MASK_ADDRESS: u64 = 0xFFFFFFFFF000;
    pub const FRAME_SIZE: u64 = 4096;
}

impl PageEntry for PtEntry {
    type TableIndex = PtIndexer;
}*/
