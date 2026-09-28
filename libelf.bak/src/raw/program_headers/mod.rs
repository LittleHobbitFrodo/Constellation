use crate::raw::types::*;

mod helpers;
pub use helpers::*;


#[derive(Clone)]
#[repr(C)]
pub struct RawProgramHeader {
    /// `p_type`: Identifies the type of segment
    /// - Segment types are explained in table 16
    pub typ: HeaderEntryType,
    /// `p_flags`: Contains segment attributes
    /// - Shown in table 17
    /// - The highest 8 bits are reserved for processor-specific use
    ///   - The next 8 bits are reserved for environment-specific use
    pub flags: HeaderEntryFlags,
    /// `p_offset`: Contains the offset of the segment in the file
    /// - In bytes
    pub offset: Offset,
    /// `p_vaddr`: Virtual address of the segment in the memory
    pub virt_addr: Address,
    /// `p_paddr`: Reserved for systems with physical addressing
    _reserved: Address,
    /// `p_filesz`: Contains the size of the file image of the segment
    /// - The in-file size must be less than or equal to `self.memory_size`
    pub file_size: XWord,
    /// `p_memsz`: Contains the size of the memory image of the segment
    pub memory_size: XWord,
    /// `p_align`: Specifies the alignment that the virtual and physical addresses must follow
    /// - Must be power of two
    pub align: XWord,
}
