use core::{cell::UnsafeCell, fmt::Debug, ptr::NonNull, sync::atomic::Ordering::{self, Relaxed}};

use crate::helpers::RwRc;



#[cfg(test)]
mod tests;

mod read_guard;
pub use read_guard::*;

mod write_guard;
pub use write_guard::*;



#[repr(C)]
pub struct RwLock<T> {
    data: UnsafeCell<T>,
    rc: RwRc,
}

unsafe impl<T: Send> Send for RwLock<T> {}
unsafe impl<T: Send + Sync> Sync for RwLock<T> {}


impl<T> RwLock<T> {


    /// Construct new `RwLock` guarding the given value
    pub const fn new(val: T) -> Self {
        Self { data: UnsafeCell::new(val), rc: RwRc::new() }
    }



    /// Locks this `RwLock` with shared read-only access. This functil will spin until a writer is deactivated
    /// - Returns `RwLockReadGuard`, which will release the lock once dropped
    ///
    /// This function also disables mutable access to this lock for at least the existence of the returned guard
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        self.rc.add_reader();
        RwLockReadGuard::new(unsafe { self.data.get().as_ref_unchecked() })
    }

    /// Attempt to acquire this lock with shared read access
    /// - This function does not block and returns immediately
    ///
    /// This function also disables mutable access to this lock for at least the existence of the returned guard
    #[inline]
    pub fn try_read(&self) -> Option<RwLockReadGuard<'_, T>> {
        match self.rc.try_add_reader() {
            Ok(_) => Some(RwLockReadGuard::new(unsafe { self.data.get().as_ref_unchecked() }/*, &self.rc*/)),
            Err(_) => None,
        }
    }


    /// Obtains exclusive mutable access to the lock. This function will spin until it can be obtained
    /// - Returns `RwLockWriteGuard`, which will release the lock once dropped
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.rc.activate_writer();
        RwLockWriteGuard::new(unsafe { self.data.get().as_mut_unchecked() })
    }


    /// Attempts to acquire exclusive access to the shared data
    ///
    /// This function does not block and returns immediately
    #[inline]
    pub fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>> {
        match self.rc.try_activate_writer() {
            Ok(_) => Some(RwLockWriteGuard::new(unsafe { self.data.get().as_mut_unchecked() }, /*&self.rc*/)),
            Err(_) => None,
        }
    }



    /// Consumes the `RwLock` and returns its inner value
    pub fn into_inner(self) -> T {
        let Self { data, .. } = self;
        data.into_inner()
    }


    /// Returns a `NonNUll` pointer to the shared data
    #[inline(always)]
    pub fn as_ptr(&self) -> NonNull<T> {
        unsafe {
            NonNull::new_unchecked(self.data.get())
        }
    }


    /// Returns the number of readers holding the lock
    /// - Given that this function uses `Relaxed` ordering, the returned value should be considered out of date
    ///   - Use the `reader_count_ordered()` function to specify the ordering
    #[inline(always)]
    pub fn reader_count(&self) -> Option<usize> {
        self.rc.reader_count()
    }

    /// Returns the number of readers holding the lock, the user specifies the atomic operation ordering
    #[inline(always)]
    pub fn reader_count_ordered(&self, order: Ordering) -> Option<usize> { self.rc.state(order) }

    /// Indicates wheter the lock is held by a writer
    /// - Given that this function uses `Relaxed` ordering, the returned value should be considered out of date
    ///   - Use the `has_writer_ordered()` function to specify the ordering
    #[inline(always)]
    pub fn has_writer(&self) -> bool { self.rc.state_raw(Relaxed) < 0 }

    /// Indicates wheter the lock is held by a writer, the user specifies the atomic operation ordering
    #[inline(always)]
    pub fn has_writer_ordered(&self, order: Ordering) -> bool { self.rc.state_raw(order) < 0 }

    /// Forcibly dereses the reader count
    ///
    /// This is useful when dealling with FFI that does not understand RAII
    ///
    /// # Safety
    /// Calling this function is extremely unsafe when not
    /// exactly paired with each call of `RwLock::read()`
    /// - For one call of `force_reader_decrement()` one
    /// `RwLockReadGuard` must not be dropped
    ///
    /// ## When misused
    /// Calling this function when all `RwLockReadGuard`s
    /// are/will be dropped will unlock the `RwLock`,
    /// therefore a writer can grab it when someone is
    /// still reading it, resulting in data races and undefined behaviour
    ///
    /// When calling this function when a writer is active, by design, this will cause an integer overflow and panic
    #[inline(always)]
    pub unsafe fn force_reader_decrement(&self) {
        unsafe {
            self.rc.remove_reader_unchecked()
        }
    }


    /// Forcibly unlocks the writer owning the exclusive access to this lock
    ///
    /// This is useful when dealling with FFI that does not understand RAII
    ///
    /// # Safety
    /// This function is extremely unsafe when called when a writer is still used.
    /// The active writer must be discarded (with `core::mem::forget()` or simillar
    /// functions) after calling this
    #[inline(always)]
    pub unsafe fn force_write_unlock(&self) {
        self.rc.deactivate_writer();
    }


    /// Returns a mutable reference to the underlying data
    #[inline]
    pub fn get_mut(&mut self) -> &mut T { self.data.get_mut() }

}

#[cfg(debug_assertions)]
impl<T: Debug> Debug for RwLock<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.try_read() {
            Some(guard) => write!(f, "RwLock {{ data: {:?}, rc: {:?} }}", *guard, self.rc),
            None => write!(f, "RwLock {{ <writer> }}"),
        }
    }
}


impl<T: Default> Default for RwLock<T> {
    #[inline(always)]
    fn default() -> Self { Self { data: UnsafeCell::new(T::default()), rc: RwRc::new() } }
}

impl<T> From<T> for RwLock<T> {
    #[inline(always)]
    fn from(value: T) -> Self { Self { data: UnsafeCell::new(value), rc: RwRc::new() } }
}

impl<T: Clone> Clone for RwLock<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            data: UnsafeCell::new(unsafe { self.data.get().as_ref_unchecked().clone() }),
            rc: RwRc::new()
        }
    }
}
