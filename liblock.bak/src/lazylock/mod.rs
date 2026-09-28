use core::marker::PhantomData;
use core::ptr::NonNull;
use core::sync::atomic::Ordering::{self, AcqRel, Acquire};

use crate::helpers::AtomicFn;
use crate::{Once, helpers::Status};

#[cfg(test)]
mod test;



#[repr(C)]
pub struct LazyLock<T, F: Fn() -> T> {
    once: Once<T>,
    init: AtomicFn<T>,
    _marker: PhantomData<F>
}


impl<T, F: Fn() -> T> LazyLock<T, F> {
    pub const fn new(init: fn() -> T) -> Self {
        Self {
            once: Once::new(),
            init: AtomicFn::new(Some(init)),
            _marker: PhantomData,
        }
    }

    /// Returns the initialization `Status` of this `LazyLock`
    #[inline(always)]
    fn get_status(this: &Self, order: Ordering) -> Status {
        this.once.get_status(order)
    }



    /// Evaluates the lazy value and returns a reference to it
    /// - Explicit `Deref` call
    pub fn get(&self) -> &T {
        match self.init.swap(None, AcqRel) {
            Some(f) => self.once.call_once(f),
            None => self.once.wait(),
        }
    }

    /// Returns a `NonNull` pointer to the underlying data
    /// - Note that the data may not be initialized
    #[inline(always)]
    pub fn as_ptr(&self) -> NonNull<T> {
        self.once.as_ptr()
    }

}


impl<T, F: Fn() -> T> core::ops::Deref for LazyLock<T, F> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &Self::Target { self.get() }
}


impl<T: Default> Default for LazyLock<T, fn() -> T> {
    /// Creates an `LazyLock<T, F>` where `F` if `T::default()`
    #[inline(always)]
    fn default() -> Self { Self::new(T::default) }
}
