use crate::raw::types::*;


#[derive(Clone, Copy)]
#[repr(u16)]
pub enum SegmentPermissions {
    /// `PF_X`: Segment is executable
    Execute = 0x1,
    /// `PF_W`: Segment is writeable
    Writeable = 0x2,
    /// `PF_R`: Segment is radable
    /// - Ignored - all segments must be readable
    Readable = 0x4,
}

impl core::fmt::Debug for SegmentPermissions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}{}{}",
            if *self as u16 & Self::Readable as u16 != 0 { 'r' } else { '-' },
            if *self as u16 & Self::Writeable as u16 != 0 { 'w' } else { '-' },
            if *self as u16 & Self::Execute as u16 != 0 { 'x' } else { '-' })
    }
}

impl SegmentPermissions {

    /// Indicates whether the segment is readable
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        (*self as u16) & Self::Readable as u16 != 0
    }

    /// Indicates whether the segment is writeable
    #[inline(always)]
    pub fn is_writeable(&self) -> bool {
        (*self as u16) & Self::Writeable as u16 != 0
    }

    /// Indicates whether this segment is executable
    /// - Executable and writeable segments are forbidden in cronos
    #[inline(always)]
    pub fn is_executable(&self) -> bool {
        (*self as u16) & Self::Execute as u16 != 0
    }

}

#[derive(Clone)]
#[repr(transparent)]
pub struct HeaderEntryFlags(u32);

impl HeaderEntryFlags {

    /// Returns the non-reserved flags
    #[inline]
    pub fn permissions(&self) -> SegmentPermissions {
        unsafe { core::mem::transmute(self.0 as u16) }
    }

    /// Returns the processor-specific flags
    #[inline]
    pub fn cpu_flags(&self) -> u32 { (self.0 >> 16) & 0xFF }

    /// Returns the environment-specific flags
    #[inline]
    pub fn env_flags(&self) -> u32 { (self.0 >> 24) & 0xFF }
}

impl core::fmt::Debug for HeaderEntryFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, r"HeaderEntryFlags {{ perms: {:?}, cpu_flags: {:x}, env_flags: {:x} }}",
        self.permissions(),
        self.cpu_flags(),
        self.env_flags())
    }
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum HeaderEntryType {
    /// `PT_NULL`: Unused entry
    Unused = 0,
    /// `PT_LOAD`: This entry should be loaded into the program's memory
    Loadable = 1,
    /// `PT_DYNAMIC`: Dynamic linking table
    DynamicTable = 2,
    /// `PT_INTERP`: Path to the interpretter
    Interpretter = 3,
    /// `PT_NOTE`: Section for notes
    NoteSection = 4,
    /// `PT_SHLIB`: Reserved
    _Reserved = 5,
    /// `PT_PHDR`: Program header table
    ProgramHeaderTable = 6,
    /// `PT_LOOS`: Environment-specific use
    EnvSpecific = 0x60000000,
}

impl core::fmt::Debug for HeaderEntryType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::Unused => {
                write!(f, "unused")?;
                if f.alternate() {
                    write!(f, " (PT_NULL)")?;
                }
            },
            Self::Loadable => {
                write!(f, "loadable")?;
                if f.alternate() {
                    write!(f, " (PT_LOAD)")?;
                }
            },
            Self::DynamicTable => {
                write!(f, "dynamic table")?;
                if f.alternate() {
                    write!(f, " (PT_DYNAMIC)")?;
                }
            },
            Self::Interpretter => {
                write!(f, "interpretter")?;
                if f.alternate() {
                    write!(f, " (PT_INTERP))")?;
                }
            },
            Self::NoteSection => {
                write!(f, "note")?;
                if f.alternate() {
                    write!(f, " (PT_NOTE)")?;
                }
            },
            Self::_Reserved => write!(f, "<reserved>")?,
            Self::ProgramHeaderTable => {
                write!(f, "program header table")?;
                if f.alternate() {
                    write!(f, " (PT_PHDR)")?;
                }
            },
            Self::EnvSpecific => {
                write!(f, "environment-specific")?;
                if f.alternate() {
                    write!(f, " (PT_LOOS)")?;
                }
            }
        }

        Ok(())
    }
}

impl HeaderEntryType {

    /// Values in inclusive range between `PROCESSOR_SPECIFIC_LOWER` and `PROCESSOR_SPECIFIC_HIGHER`
    /// are deemed processor specific
    pub const PROCESSOR_SPECIFIC_LOWER: u32 = 0x70000000;

    /// Values in inclusive range between `PROCESSOR_SPECIFIC_LOWER` and `PROCESSOR_SPECIFIC_HIGHER`
    /// are deemed processor specific
    pub const PROCESSOR_SPECIFIC_HIGHER: u32 = 0x7fffffff;

    pub fn is_processor_spceific(&self) -> bool {
        match *self as u32 {
            Self::PROCESSOR_SPECIFIC_LOWER..Self::PROCESSOR_SPECIFIC_HIGHER => true,
            _ => false,
        }
    }

}
