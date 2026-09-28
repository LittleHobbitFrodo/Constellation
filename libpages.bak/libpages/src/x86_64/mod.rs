

mod table;
pub use table::*;

use crate::Either;

pub(crate) mod virt_addr;

pub mod levels;

pub mod address_space;

/*pub struct PageTable<Ent: PageEntryUnion>();


/// Marks any kind of page entry
pub(crate) trait PageEntryMarker
where Self: Sized + Copy + Clone { }

pub(crate) trait PageEntryUnion
where Self: PageEntryMarker {

    /*type RegularEntry: RegularPageEntry;

    type SizedEntry: SizedPageEntry;

    fn get(&self) -> Either<Self::RegularEntry, Self::SizedEntry>;*/

}

/// Marks any regular page entry
pub trait RegularPageEntry
where Self: PageEntryMarker { }

/// Marks any sized page entry
pub trait SizedPageEntry
where Self: PageEntryMarker {}*/
