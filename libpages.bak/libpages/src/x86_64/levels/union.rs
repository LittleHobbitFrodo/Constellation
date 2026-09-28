use core::ptr::NonNull;

use crate::{levels::{Pml4Entry, sized::PdPtSizedEntry}, virt_addr::{PdIndexer, PdPtIndexer, PtIndexer}};

use crate::Either;

use super::{PageEntryUnion, PageEntry, PdptEntry};


const BIT_PS: u64 = 0b1 << 7;


macro_rules! generate_union_entry {
    ($entry_name:ident, $regular:ident, $sized:ident, $indexer:ident) => {

        /// Page entry located in any `PageTable` and is used to
        /// distinguish between regular and sized entries
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct $entry_name(u64);

        impl PageEntry for $entry_name {
            type TableIndex = $indexer;
        }

        impl PageEntryUnion for $entry_name {

            type Regular = PdptEntry;

            type Sized = PdPtSizedEntry;

            fn get_ref(&self) -> Either<&Self::Regular, &Self::Sized> {
                let ptr = NonNull::from(&self.0);

                if self.0 & BIT_PS == 0 {
                    Either::Regular(unsafe { ptr.cast().as_ref() })
                } else {
                    Either::Sized(unsafe { ptr.cast().as_ref() })
                }
            }

            fn get_mut(&mut self) -> Either<&mut Self::Regular, &mut Self::Sized> {
                let ptr = NonNull::from(&self.0);

                if self.0 & BIT_PS == 0 {
                    Either::Regular(unsafe { ptr.cast().as_mut() })
                } else {
                    Either::Sized(unsafe { ptr.cast().as_mut() })
                }
            }

            fn get(&self) -> Either<Self::Regular, Self::Sized> {
                if self.0 & BIT_PS == 0 {
                    Either::Regular(unsafe { core::mem::transmute_copy(&self.0) })
                } else {
                    Either::Sized(unsafe { core::mem::transmute_copy(&self.0) })
                }
            }
        }

    };
}


//  Pml4UnionEntry does not exist because its sized variant does not exist

generate_union_entry!(PdPtUnionEntry, PdPtEntry, SizedPdPtEntry, PdPtIndexer);
generate_union_entry!(PdUnionEntry, Pdentry, SizedPdEntry, PdIndexer);
generate_union_entry!(PtUnionEntry, PtEntry, PtSizedEntry, PtIndexer);
