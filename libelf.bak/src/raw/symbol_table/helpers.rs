
#[repr(u8)]
pub enum SymbolAttributes {
    /// `STB_LOCAL`: Not visible outside the object file
    Local = 0,
    /// `STB_GLOBAL`: Visible to all object files
    Global = 1,
    /// `STB_WEAK`: Global symbol, but with lower precedence than global symbols
    WeakGlobal = 2,
    /// `STB_LOOS`: Environment-specific use
    EnvSpecific = 10,
    /// `STB_HIOS`: **?**
    _Hios = 12,
    /// `STB_LOPROC`: Processor-specific use
    ProcessorSpecific = 13,
    /// `STB_HIPROC`: **?**
    _Hiproc = 15,
}

#[repr(u8)]
pub enum SymbolType {
    /// `STT_NOTYPE`: No type specified - an absolute symbol
    Unspecified = 0,
    /// `STT_OBJECT`: Data object
    DataObject = 1,
    /// `STT_FUNC`: Function entry point
    Fn = 2,
    /// `STT_SECTION`: Symbol associated with section
    Section = 3,
    /// `STT_FILE`: Source file associated with the object file
    SourceFile = 4,
    /// `STT_LOOS`: Environemt-specific use
    EnvSpecific = 10,
    /// `STT_HIOS`: **?**
    _Hios = 12,
    /// `STT_LOPROC`: Processor-specific use
    ProcessorSpecific = 13,
    /// `STT_HIPROC`: **?**
    _Hiproc = 15,
}
