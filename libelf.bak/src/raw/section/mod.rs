

use crate::raw::types::*;

mod helpers;
pub(crate) use helpers::*;

#[repr(C)]
pub struct RawSectionHeader {
    /// `sh_name`: Offset in bytes to the sectionname
    /// - Relatie to the start of the section name string table
    name: Word,
    /// `sh_type`: Identifies the section type
    /// - Processor independent
    /// - Possible values in table 8
    typ: SectionType,
    /// `sh_flags`: Identifies the attributes for this section
    /// - Processor independent
    /// - Possible values in table 9
    flags: SectionFlags,
    /// `sh_addr`: Virtual address of the beginning of the section in memory
    /// - NULL if not allocated into the memory of the loaded program
    address: Address,
    /// `sh_offset`: Offset of the beginning of the section contents in the file
    /// - In bytes
    file_offset: Offset,
    /// `sh_size`: The size of the section in bytes
    /// - This is the amount of space occupied in the file
    /// - Exception: Not valid for `SHT_NOBITS` sections
    section_size: XWord,
    /// `sh_link`: Contains an section index for associated section
    /// - Explained in table 10
    link: SectionType,
    /// `sh_info`: Contains extra information about the file
    /// - Explained in table 11
    info: SectionType,
    /// `sh_addralign`: Contains required alignment of the section
    /// - Must be power of two
    address_align: XWord,
    /// `sh_entsize`: Sized of entries, if section has a table and has fixed-size entries
    /// - Otherwise set to zero
    entry_size: XWord,
}
