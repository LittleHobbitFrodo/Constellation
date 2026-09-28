use crate::levels::TableIndexer;


/// Indexes `PDPT` layer tables
#[repr(transparent)]
#[derive(Clone)]
pub struct PdPtIndexer(u16);

impl TableIndexer for PdPtIndexer {
    #[inline(always)]
    unsafe fn new_unchecked(index: u16) -> Self { Self(index) }

    #[inline(always)]
    fn as_u16(&self) -> u16 { self.0 }
}



/// Indexes `PD` layer tables
#[repr(transparent)]
#[derive(Clone)]
pub struct PdIndexer(u16);

impl TableIndexer for PdIndexer {

    #[inline(always)]
    unsafe fn new_unchecked(index: u16) -> Self { Self(index) }

    #[inline(always)]
    fn as_u16(&self) -> u16 { self.0 }
}


/// Indexes `PT` layer tables
#[repr(transparent)]
#[derive(Clone)]
pub struct PtIndexer(u16);

impl TableIndexer for PtIndexer {

    #[inline(always)]
    unsafe fn new_unchecked(index: u16) -> Self { Self(index) }

    #[inline(always)]
    fn as_u16(&self) -> u16 { self.0 }
}
