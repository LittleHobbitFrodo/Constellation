//! Raw type definitions


/// In-memory representation of the ELF address (`u64`)
/// - Size: 8, Align: 8
pub(crate) type Address = u64;

/// Type indicating in-file offset (`u64`)
/// - Size: 8, Align: 8
pub(crate) type Offset = u64;

/// Small/half unsigned integer (`u16`)
/// - Size: 2, Align: 2
pub(crate) type Half = u16;

/// Unsigned integer (`u32`)
/// - Size: 4, Align: 4
pub(crate) type Word = u32;

/// Signed integer (`i32`)
/// - Size: 4, Align: 4
pub(crate) type SWord = i32;

/// Unsigned long integer (`u64`)
/// - Size: 8, Align: 8
pub(crate) type XWord = u64;

/// Signed long integer (`i64`)
/// - Size: 8, Align: 8
pub(crate) type XSWord = i64;

/// Unsigned small integer (`u8`)
/// - Size: 1, Align: 1
pub(crate) type Byte = u8;
