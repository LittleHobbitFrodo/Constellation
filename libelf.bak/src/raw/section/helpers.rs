
#[repr(u32)]
pub enum SectionType {
    /// `SHT_NULL`: Unused section header
    Unused = 0,
    /// `SHT_PROGBITS`: Contains information defined by the program
    ProgramBits = 1,
    /// `SHT_SYMTAB`: Contains a linker symbol table
    LinkerSymbolTable = 2,
    /// `SHT_STRTAB`: Contains a string table
    StringTable = 3,
    /// `SHT_RELA`: Contains a `Rela` relocation entries
    RelocationRela = 4,
    /// `SHT_HASH`: Contains a symbol hash table
    SymbolHashTable = 5,
    /// `SHT_DYNAMIC`: Contains dynamic linking tables
    DynLinkTables = 6,
    /// `SHT_NOTE`: Contains note information
    Notes = 7,
    /// `SHT_NOBITS`: Contains uninitialized space
    /// - Does not accupy anpy space in the file
    UninitializedSpace = 8,
    /// `SHT_REL`: Contains `Rel` type relocation entries
    RelocationRel = 9,
    /// `SHT_SHLIB`: Reserved
    _Reserved = 10,
    /// `SHT_DYNSYM`: Contains Dynamic loader symbol table
    DynLoaderSymbols = 11,
    /// `SHT_LOOS`: Environment-specific use
    EnvironmentSpecific = 0x60000000,
    /// `SHT_HIOS`: **?**
    _Hios = 0x6FFFFFFF,
    /// `SHT_LOPROC`: Processor specific use
    ProcessorSpecific = 0x70000000,
    /// `SHT_HIPROC`: **?**
    _HiProc = 0x7FFFFFFF,
}

#[repr(u64)]
pub enum SectionFlags {
    /// `SHF_WRITE`: Sections contains writeable data
    Writeable = 0x1,
    /// `SHF_ALLOC`: Section is allocated in the memory image of the program
    Allocated = 0x2,
    /// `SHF_EXECINSTR`: Section contains executable instructions
    Executable = 0x4,
    /// `SHF_MASKOS`: Environment-specific use
    EnvSpecific = 0x0F000000,
    /// `SHF_MASKPROC`: Processor specific use
    ProcessorSpecific = 0xF0000000,
}
