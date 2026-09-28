use core::{ptr::NonNull, range::Range};
use core::num::NonZero;

use crate::raw::program_headers::{HeaderEntryFlags, HeaderEntryType, RawProgramHeader, SegmentPermissions};


/// Program headers are used only to load executable files or shared objects
#[repr(transparent)]
pub struct ProgramHeader {
    raw: RawProgramHeader,
}

impl core::fmt::Debug for ProgramHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let virt = self.try_virtual_address().map(|x| x.as_ptr() ).unwrap_or(core::ptr::null_mut());
        write!(f, "ProgramHeader {{ type: {:?}, flags: {:?}, file_range: {:?}, mem_range: {:p}..{:p}, align: 0x{:x} }}",
            self.entry_type(),
            self.flags(),
            self.file_offset(),
            virt, unsafe { virt.add(self.size_in_memory()) },
            self.try_virtual_address_alignment().map(|x| x.get()).unwrap_or(0))
    }
}

impl ProgramHeader {

    /// Indicates whether this entry is initialized
    pub fn is_null(&self) -> bool {
        unsafe {
            NonNull::slice_from_raw_parts(NonNull::from(&self.raw).cast::<u8>(), size_of::<Self>()).as_ref()
        }.iter().all(|x| *x == 0)
    }

    /// Indicates whether this entry should be loaded into the program's memory
    #[inline]
    pub fn is_loadable(&self) -> bool { matches!(self.entry_type(), HeaderEntryType::Loadable) }

    /// Returns the permissions of this segment
    #[inline]
    pub fn permissions(&self) -> SegmentPermissions { self.raw.flags.permissions() }

    /// Returns the type of this entry
    #[inline]
    pub fn entry_type(&self) -> HeaderEntryType {self.raw.typ.clone() }

    /// Returns segment flags/attributes
    #[inline]
    pub fn flags(&self) -> HeaderEntryFlags { self.raw.flags.clone() }

    /// Returns the offset of this section within the file
    #[inline]
    pub fn file_offset(&self) -> usize { self.raw.offset as usize }

    /// Returns the virtual address to load the segment to
    /// - Panics if the virtual address is null
    #[inline]
    pub fn virtual_address(&self) -> NonNull<u8> {
        NonNull::new(self.raw.virt_addr as usize as *mut u8).expect("program header entry's virtual address is NULL")
    }

    /// Returns the virtual address to load the segment to it it is non-null
    #[inline]
    pub fn try_virtual_address(&self) -> Option<NonNull<u8>> {
        NonNull::new(self.raw.virt_addr as usize as *mut u8)
    }

    /// Returns the virtual address to load the segment to without checking if it is null
    ///
    /// # Safety
    /// Use only if you can guarantee that the underlying virtual address is not null
    #[inline]
    pub unsafe fn virtual_address_unchecked(&self) -> NonNull<u8> {
        unsafe { NonNull::new_unchecked(self.raw.virt_addr as usize as *mut u8) }
    }

    /// Returns the range of this segment within the file
    #[inline]
    pub fn file_range(&self) -> Range<usize> {
        Range { start: self.raw.offset as usize, end: self.raw.offset as usize + self.raw.file_size as usize }
    }


    /// Returns the size of the segment inside of the file
    /// - This number can be zero in case of uninitialized segments (like for example the `.bss` section)
    /// - This number must be less than or equal to `self.size_in_memory()`
    #[inline]
    pub fn size_in_file(&self) -> usize { self.raw.file_size as usize }

    /// Returns the size of the loaded segment in memory
    /// - This number must be greater than or equal to `self.size_in_file()`
    /// - This number can be zero if the entry is NULL
    #[inline]
    pub fn size_in_memory(&self) -> usize {
        self.raw.memory_size as usize
    }

    /// Returns the alignment requirements for the virtual address to load the segment to
    pub fn virtual_address_alignment(&self) -> NonZero<usize> {
        let align = self.raw.align as usize;
        if align.is_power_of_two() {
            unsafe { NonZero::new_unchecked(align) }
        } else {
            panic!("program header entry's virt addr alignment is not power of two");
        }
    }

    /// Returns the alignment requirements for the virtual address
    /// - Does not check if it is power of two
    ///
    /// # Safety
    /// Use only if you can guarantee that the underlying value is power of two
    #[inline]
    pub unsafe fn virtual_address_alignment_unchecked(&self) -> NonZero<usize> {
        unsafe { NonZero::new_unchecked(self.raw.align as usize) }
    }

    /// Returns the alignment requirements for the virtual address
    /// - Returns `None` if it is not power of two
    #[inline]
    pub fn try_virtual_address_alignment(&self) -> Option<NonZero<usize>> {
        let align = self.raw.align as usize;
        if align.is_power_of_two() {
            Some(unsafe { NonZero::new_unchecked(align) })
        } else {
            None
        }
    }


    //#[cfg(feature = "raw")]
    /// Returns the raw program header structure
    #[inline]
    pub fn raw_program_header(&self) -> &RawProgramHeader { &self.raw }


}
