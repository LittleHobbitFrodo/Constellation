use crate::raw::section::RawSectionHeader;



#[repr(transparent)]
pub struct SectionEntry {
    raw: RawSectionHeader,
}
