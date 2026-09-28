use core::num::NonZero;

use crate::raw::types::*;

mod helpers;
pub use helpers::*;


#[repr(C)]
pub struct SymbolTable {
    /// `st_name`: Contains the offset to the symbol name
    /// - Symbol has no name if zero
    /// - Relative to the start of the symbol table
    /// - In bytes
    name: Option<NonZero<Word>>,
    /// `st_info`: Contains symbol type and its attributes (scope)
    /// - Attributes are listed in table 14
    /// - Symbol types are listed in table 15
    info: SymbolInfo,
    /// `st_other`: Reserved - must be zero
    _reserved: u8,
    /// `st_shndx`: Contains section index of the section in which the symbol is defined
    /// - Undefined symbols contains `SHN_UNDEF`
    /// - Absolute symbols contains `SHN_ABS`
    /// - Common symbols contains `SHN_COMMON`
    sec_table_index: Half,
    /// `st_value`: Contains the symbol value
    /// - May be an absolute address or relocatable address
    ///   - Contains alignment and a section-relative offset for defined relocatable symbols
    /// - For executable and shared object files it contains virtual address defined relocatable symbols
    symbol_value: Address,
    /// `st_size`: Contains the size of the object
    /// - Zero if size is unknown or it is unsized
    object_size: Option<NonZero<XWord>>,
}


/// Represents the `st_info` filed in the symbol table
#[repr(transparent)]
pub struct SymbolInfo(u8);

impl SymbolInfo {
    /// Returns the symbol attributes
    #[inline]
    fn attributes(&self) -> SymbolAttributes {
        unsafe { core::mem::transmute(self.0 >> 4) }
    }

    /// Returns the type of the symbol
    #[inline]
    fn symbol_type(&self) -> SymbolType {
        unsafe { core::mem::transmute(self.0 & 0x0F) }
    }
}
