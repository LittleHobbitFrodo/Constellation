use crate::raw::types::*;


#[repr(C)]
pub struct RawDynamicTableEntry {
    /// `d_tag`: Identifies the type of the dynamic entry
    /// - Dyn table entry types are explained in table 18
    tag: Tag,
    un: Value,
}



pub union Value {
    /// Represents an integer value
    value: XWord,
    /// Represents program link-time virtual addresses
    /// - Must be relocated to match the object's actual load address
    ptr: Address
}

#[repr(i64)]
pub enum Tag {
    /// `DT_NULL`: Ignored entry
    Ignored = 0,
    /// `DT_NEEDED`: String table offset to a name of a needed library
    NeededLib = 1,
    /// `DT_PLTRELSZ`: Total size of relocation entries associated with the procedure linkage table
    /// - In bytes
    RelEntriesSize = 2,
    /// `DT_PLTGOT`: Address associated with the linkage table
    /// - Processor dependent
    LinkageTableAddr = 3,
    /// `DT_HASH`: Holds the address of the symbol hash table
    SymbolHashTable = 4,
    /// `DT_STRTAB`: Holds the address of the string table
    StringTable = 5,
    /// `DT_SYMTAB`: Holds the address of the symbol table
    SymbolTable = 6,
    /// `DT_RELA`: Holds the address of the relocation table
    RelaTable = 7,
    /// `DT_RELASZ`: Holds the size of the relocation table
    /// - In bytes
    RelaTableSize = 8,
    /// `DT_RELAENT`: Holds the size of any relocation table entry
    /// - In bytes
    RelaEntrySize = 9,
    /// `DT_STRSZ`: Hold the size of the string table
    /// - In bytes
    StringTableSize = 10,
    /// `DT_SYMENT`: Holds the size of a symbol table entry
    SymbolEntrySize = 11,
    /// `DT_INIT`: Holds the address of the initialization function
    FnInit = 12,
    /// `DT_FINI`: Holds the address of the termination function
    FnTerminate = 13,
    /// `DT_SONAME`: Offset to a null-terminated string table indicating the name of the shared object
    /// - In bytes
    ///
    /// **TODO**: docs
    ObjectName = 14,
    /// `DT_RPATH`: Offset to a null-terminated library search path string
    ///
    /// **TODO**: docs
    LibPath = 15,
    /// `DT_SYMBOLIC`: Presence of this element alters dynamic linker's behaviour:
    /// - Starts symboll lookup within itself (the shared object)
    ///   - If the symbol is not found, continues within the executable as usual
    SymbolicReferences = 16,
    /// `DT_REL`: Similar to `RelocationTable` (`DT_RELA`), but has implicit addends
    RelTable = 17,
    /// `DT_RELSZ`: Hlds the size of the `Rel` table
    /// - In bytes
    RelTableSize = 18,
    /// `DT_RELENT`: Holds the size of any `Rel` entry
    /// - In bytes
    RelEntrySize = 19,
    /// `DT_PLTREL`: Specifies the type of relocation entry to which the procedure linkage table refer
    ///
    /// **TODO**: docs
    RelEntryType = 20,
    /// `DT_DEBUG`: Used for debugging
    /// - ABI independent
    Debug = 21,
    /// `DT_TEXTREL`: Presence of this element enables the loader to modify any non-writeable segment
    /// - As specified by the segment permissions
    TextRelocations = 22,
    /// `DT_JMPREL`: Holds the address of relocation entries associated solely with the procedure linkage table
    /// - If this entry is present, the related entries of types `DT_PLTRELSZ` and `DT_PLTREL` must also be present
    LinkageTablePtr = 23,
    /// `DT_LOPROC`: Processor-specific semantics
    ProcessorSpecific = 0x70000000,
    /// `DT_HIPROC`: **?**
    _Hiproc = 0x7fffffff,
}
