use crate::raw::{symbol_table::SymbolType, types::*};

#[repr(C)]
pub struct Rel {
    /// `r_offset`: Indicates the location at which the relocation should be applied
    /// - For relocatable file, this is an byte offset from the beginning of the section to the storage unit being relocated
    /// - For executable file or shrared object, this is the virtual address of the section to the storage unit being relocated
    offset: Address,
    /// `r_info`: Contains symbol table index and relocation type
    /// - The symbol table index contains index of the table used for the relocation
    /// - Relocation types are processor specific
    /// -
    info: RelOffset,
}

#[repr(C)]
pub struct Rela {
    /// `r_offset`: Indicates the location at which the relocation should be applied
    /// - For relocatable file, this is an byte offset from the beginning of the section to the storage unit being relocated
    /// - For executable file or shrared object, this is the virtual address of the section to the storage unit being relocated
    offset: Address,
    /// `r_info`
    info: RelOffset,
    /// `r_addend`: Specified a constant used to compute the final value
    /// - **?**
    const_expr: XSWord,
}

#[repr(transparent)]
pub struct RelOffset(u64);

impl RelOffset {
    /// Returns the symbol table index
    #[inline]
    pub fn symbol_table_index(&self) -> u32 { (self.0 >> 32) as u32 }

    /// Returns the symbol type
    #[inline]
    pub fn symbol_type(&self) -> u32 { self.0 as u32 }
}
