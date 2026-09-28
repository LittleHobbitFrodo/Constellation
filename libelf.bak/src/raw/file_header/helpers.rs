//! Introduces helper structs and enums for the `FileHeader`

/// Represents the class of the ELF file (object data structure size)
/// - Cronos allows only 64-bit executables **?**
#[repr(u8)]
pub enum FileClass {
    /// `ELFCLASS32`: 32-bit ELF data structures
    Object32Bit = 1,
    /// `ELFCLASS64`: 64-bit ELF data structures
    Object64Bit = 2,
}

/// Represents the data encoding of the ELF file
/// - Cronos allows only little endian
#[repr(u8)]
pub enum DataEncoding {
    /// `ELFDATA2LSB`: Little endian (lsb)
    LittleEndian = 1,
    /// `ELFDATA2MSB`: Big endian (msb)
    BigEndian = 2,
}


#[repr(u8)]
pub enum OsAbi {
    /// `ELFOSABI_SYSV `: System V ABI
    SystemVAbi = 0,
    /// `ELFOSABI_HPUX`: HP-UX operating system
    HpUx = 1,
    /// `ELFOSABI_STANDALONE`: Standalone/embedded application
    StandAlone = u8::MAX,
}


#[repr(u16)]
pub enum ObjectFileType {
    /// `ET_NONE`: No file type
    None = 0,
    /// `ET_REL`: Relocatable object file
    Relocatable = 1,
    /// `ET_EXEC`: Executable file
    Executable = 2,
    /// `ET_DYN`: Shared object file
    SharedObject = 3,
    /// `ET_CORE`: Core file **?**
    CoreFile = 4,
    /// `ET_LOOS`: Environment specific use
    EnvSpecific = 0xFE00,
    /// `ET_HIOS`: **?**
    _Hios = 0xFEFF,
    /// `ET_LOPROC`: Processor specific use
    ProcessorSpecific = 0xFF00,
    /// `ET_HIPROC`: **?**
    _Hiproc = 0xFFFF,
}

#[repr(u16)]
pub enum MachineType {
    ToDo,
}
