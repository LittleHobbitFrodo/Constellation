//! Provides the ELF file header
//! - File header is always located at the beginning of the ELF file

use crate::raw::types::*;

mod helpers;
pub(crate) use helpers::*;

#[repr(C)]
pub struct FileHeader {
    /// `e_ident`: Byte array used to identify an ELF file and provide info about data representation
    ///
    /// Some bytes have defined meaning, some are reserved and should be set to zero
    pub identification: FileIdentification,
    /// `e_type`: Identifies the object file type
    /// - Processor independent
    /// - Possible values are in table 6
    pub file_type: ObjectFileType,
    /// `e_machine`: Identifies specific architecture
    pub architecture: MachineType,
    /// `e_version`: Identifies the version of the object file format
    /// - Currently set to `EV_CURRENT = 1`
    pub obj_file_version: Word,
    /// `e_entry`: Contains the virtual address of the entry point function
    /// - NULL if there is no entry point
    pub entry_point: Address,
    /// `e_phoff`: Contains the file offset of the program header table
    /// - In bytes
    pub prog_header_off: Offset,
    /// `e_shoff`: Contains the file offset of the section header table
    pub sec_header_off: Offset,
    /// `e_flags`: Processor specific flags
    pub cpu_flags: Word,
    /// `e_ehsize`: Contains the size of the ELF header in bytes
    pub file_header_size: Half,
    /// `e_phentsize`: Program table entry size in bytes
    pub prog_header_entry_size: Half,
    /// `e_phnum`: Program table entry count
    pub prog_header_entry_count: Half,
    /// `e_shentsize`: Section header table entry size in bytes
    pub sec_header_entry_size: Half,
    /// `e_shnum`: Section header entry count
    pub sec_header_entry_count: Half,
    /// `e_shstrndx`: Contians the section table index of the section containing section name string table
    pub sec_name_str_table_idx: Half,
}

impl FileHeader {

    /// Magic number used to identify 64-bit ELF file
    pub const MAGIC: [u8; 4] = FileIdentification::MAGIC;

    /// Checks if the magic number is valid
    #[inline]
    pub fn check_magic(&self) -> bool { Self::MAGIC == self.identification.magic }



}

#[repr(C, align(16))]
pub(crate) struct FileIdentification {
    /// `EI_MAG0-3`: Contains the magic number
    pub magic: [u8; 4],
    /// `EI_CLASS`: Identifies the class of the object file
    /// - Possible values in table 3
    pub file_class: FileClass,
    /// `EI_DATA`: Specifies the data encoding of the ELF file
    /// - For convenience should match encoding of the running program
    pub data_encoding: DataEncoding,
    /// `EI_VERSION`: ELF file version
    /// - Currently holds the value of `EV_CURRENT = 1` **?**
    pub version: u8,
    /// `EI_OSABI`: OS/ABI identification
    /// - Values defined in table 5
    pub abi: OsAbi,
    /// `EI_ABIVERSION`: Identifies the version of used ABI
    /// - Used to distinguish among incompatible versions
    /// - Dependent on the `EI_OSABI` field
    /// - Binaries using SYSV (3rd edition) contains 0
    pub abi_version: u8,
    /// Reserved bytes - should be **ZERO**
    _reserved: [u8; 6],
    ///
    pub file_ident_size: u8,
}

impl FileIdentification {

    pub const MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];

}
