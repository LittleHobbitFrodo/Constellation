

use core::num::NonZero;
use core::ptr::NonNull;

use crate::raw;
use crate::raw::program_headers::RawProgramHeader;
use crate::{reinterpret_slice, reinterpret_slice_at, reinterpret_slice_as_slice_at};

use raw::file_header::{DataEncoding, FileClass, FileHeader};


mod program_headers;
pub use program_headers::ProgramHeader;
pub use crate::raw::program_headers::{HeaderEntryFlags, HeaderEntryType, SegmentPermissions};

mod section;

mod prog_hdr_iter;
pub use prog_hdr_iter::*;

/// Representation of a parsed position-dependent ELF64 file
pub struct Elf64<'l>
where Self: 'l {
    pub(crate) slice: &'l [u8],
    file_header: &'l FileHeader,
    /// Pointer to the program header table
    /// - The size and count of the entries are stored in the `file_handler`
    prog_header_table: NonNull<ProgramHeader>,
}


impl<'l> Elf64<'l> {

    /// Returns the underlying buffer that have been
    /// passed when this instance was created
    #[inline]
    pub fn as_bytes(&self) -> &[u8] { self.slice }

    /// Returns the program entry point
    #[inline]
    pub fn program_entry(&self) -> Option<NonNull<u8>> {
        NonNull::new(self.file_header.entry_point as usize as *mut u8)
    }

    /// Returns an iterator over the program headers
    #[inline]
    pub fn program_headers(&'l self) -> ProgramHeaderIterator<'l> {
        ProgramHeaderIterator::new(self.prog_header_table, self.file_header.prog_header_entry_count, self.file_header.prog_header_entry_size)
    }

    /// Parses position-dependent ELF64 file
    #[inline(never)]
    pub fn parse(elf: &'l [u8]) -> Result<Self, ElfParseError> {

        //  Get the file header
        let file_header = match reinterpret_slice::<FileHeader>(elf) {
            Some(r) => r,
            None => return Err(ElfParseError::FileHeaderNotPresent),
        };

        //  Verify the file type
        if !file_header.check_magic() { return Err(ElfParseError::InvalidHeader) }

        //  assert little endian encoding
        match file_header.identification.data_encoding {
            DataEncoding::LittleEndian => {},
            _ => return Err(ElfParseError::InvalidDataEncoding),
        }

        //  assert 64-bit file
        match file_header.identification.file_class {
            FileClass::Object64Bit => {},
            _ => return Err(ElfParseError::InvalidFileClass)
        }

        //  Check entry point presence
        if file_header.entry_point == 0 {
            return Err(ElfParseError::EntryPointNotPresent)
        }

        //  check program header table presence
        if file_header.prog_header_off == 0 {
            return Err(ElfParseError::ProgramHeaderTableNotPresent)
        }

        //  Check the size of program header entries
        if (file_header.prog_header_entry_size as usize) < size_of::<RawProgramHeader>() {
            return Err(ElfParseError::IncompatibleProgramHeaderSize)
        }

        let prog_header_table = if (file_header.prog_header_off as usize + (file_header.prog_header_entry_size * file_header.prog_header_entry_count) as usize) < elf.len() {
            unsafe {
                NonNull::new_unchecked(elf.as_ptr() as *mut u8).add(file_header.prog_header_off as usize)
                    .cast::<ProgramHeader>()
            }
        } else {
            return Err(ElfParseError::ProgramHeaderTableNotPresent)
        };

        Ok(Self {
            slice: elf,
            file_header,
            prog_header_table,
        })

    }


    #[cfg(feature = "raw")]
    /// Returns the raw ELF64 file header
    #[inline]
    pub fn raw_file_header(&'l self) -> &'l FileHeader { self.file_header }

}


#[derive(Clone, Debug)]
pub enum ElfParseError {
    /// The file header is not present and the file cannot be treated as ELF64
    FileHeaderNotPresent,
    /// The file header was not matched and the file cannot be treated as ELF64
    InvalidHeader,
    /// The loader supports only little endian files
    InvalidDataEncoding,
    /// The loader supports only 64-bit ELFs
    InvalidFileClass,
    /// The entrypoint to this executable is absent
    EntryPointNotPresent,
    /// Program headers are absent
    ProgramHeaderTableNotPresent,
    /// The size of program headers is too small
    IncompatibleProgramHeaderSize,
}
