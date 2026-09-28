use core::{marker::PhantomData, ops::{Deref, DerefMut, Index}};

use libmem::PAGE_TABLE_ENTRY_COUNT;

use crate::levels::{TableIndexer, PageEntryUnion};


/// A page table used in virtual address translation
#[derive(Clone)]
#[allow(private_bounds)]
#[repr(transparent)]
pub struct PageTable<Ent: PageEntryUnion>([Ent; PAGE_TABLE_ENTRY_COUNT]);


#[allow(private_bounds)]
impl<Ent: PageEntryUnion> PageTable<Ent> {

    /// Constructs a new `PageTable`
    ///
    /// > TODO: COnsider API change?
    #[inline(always)]
    pub const fn new(raw: [Ent; PAGE_TABLE_ENTRY_COUNT]) -> Self {
        Self(raw)
    }


    /// Indexes the table using the indexer
    /// - All table indexers are guaranteed to not overflow the table
    #[inline(always)]
    #[allow(private_interfaces)]
    pub fn index(&self, index: Ent::TableIndex) -> &Ent {
        unsafe {
            self.0.get_unchecked(index.as_u16() as usize)
        }
    }

    /// Indexes the table using the indexer
    /// - All table indexers are guaranteed to not overflow the table
    #[inline(always)]
    #[allow(private_interfaces)]
    pub fn index_mut(&mut self, index: Ent::TableIndex) -> &mut Ent {
        unsafe {
            self.0.get_unchecked_mut(index.as_u16() as usize)
        }
    }


    /*/// Returns a reference to the entry at the given position
    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<&Ent> {
        self.0.get(index)
    }

    ///
    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Ent> {
        self.0.get_mut(index)
    }

    /// Indexes the table without doing the bounds checking
    ///
    /// # Safety
    /// It is up to the caller to guarantee that the `index` is less than the
    /// `PAGE_TABLE_ENTRY_COUNT` constant. Violating this rule will introduce
    /// undefined behaviour
    pub unsafe fn get_unchecked(&self, index: usize) -> &Ent {
        unsafe { self.0.get_unchecked(index) }
    }*/
}


impl<Ent: PageEntryUnion> Deref for PageTable<Ent> {
    type Target = [Ent; PAGE_TABLE_ENTRY_COUNT];

    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<Ent: PageEntryUnion> DerefMut for PageTable<Ent> {

    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}



/*impl<Ent: PageEntry> Index<Ent::TableIndex> for PageTable<Ent> {
    type Output = Ent;

    #[inline(always)]
    #[allow(private_interfaces)]
    fn index(&self, index: Ent::TableIndex) -> &Self::Output {
        unsafe {
            self.0.get_unchecked(index.index() as usize)
        }
    }
}*/
