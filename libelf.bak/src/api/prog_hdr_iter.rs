use crate::api::program_headers::ProgramHeader;
use core::{marker::PhantomData, num::NonZero, ptr::NonNull};



pub struct ProgramHeaderIterator<'l>
where Self: 'l {
    /// Pointer to the base of the program header table
    base: NonNull<u8>,
    //  /// Index of the pointer - where the pointer really is
    //  index: u16,
    /// The count of the elements within the table
    count: u16,
    /// The size of each entry: size is defined per-file
    entry_size: u16,
    /// Tells the compiler to stop bothering me about that lifetime
    _marker: PhantomData<&'l ProgramHeader>,
}

impl<'l> ProgramHeaderIterator<'l> {
    pub(crate) fn new(base: NonNull<ProgramHeader>, count: u16, entry_size: u16) -> Self {
        Self { base: NonNull::from(base).cast(), count, entry_size, _marker: PhantomData }
    }
}


impl<'l> Iterator for ProgramHeaderIterator<'l> {

    type Item = &'l ProgramHeader;

    fn next(&mut self) -> Option<Self::Item> {
        match self.count.checked_sub(1) {
            Some(c) => self.count = c,
            None => {
                self.count = 0;
                return None
            }
        }

        let r = unsafe { self.base.cast::<ProgramHeader>().as_ref() };

        unsafe {
            self.base = self.base.add(self.entry_size as usize);
        }

        Some(r)
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let n = n as u16;

        match self.count.checked_sub(n) {
            Some(c) => self.count = c,
            None => {
                self.count = 0;
                return None
            }
        }

        unsafe { self.base = self.base.add((self.entry_size as usize).saturating_mul(n as usize)) };

        self.next()
    }
}
