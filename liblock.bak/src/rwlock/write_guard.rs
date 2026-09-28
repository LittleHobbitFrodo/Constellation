use core::{fmt::{Debug, Display}, ptr::NonNull, sync::atomic::Ordering::{self, Acquire, Release}};
use crate::{RwLockReadGuard, helpers::RwRc};

use super::RwLock;


pub struct RwLockWriteGuard<'rwlock, T>(&'rwlock mut T)
where T: 'rwlock;

unsafe impl<T: Send + Sync> Send for RwLockWriteGuard<'_, T> {}
unsafe impl<T: Send + Sync> Sync for RwLockWriteGuard<'_, T> {}


impl<'rwlock, T> RwLockWriteGuard<'rwlock, T> {
    pub(super) const fn new(data: &'rwlock mut T) -> Self {
        Self(data)
    }

    /// Leaks the `RwLockWriteGuard` and returns a mutable reference to the shared data
    ///
    /// Note that this will lock the `RwLock` in a state where any reader or writers cannot access it
    #[inline]
    pub fn leak<'l>(self) -> &'l mut T
    where 'rwlock: 'l {
        let data = unsafe { NonNull::new_unchecked(self.0 as *mut T).as_mut() };
        core::mem::forget(self);
        data
    }


    /// Downgrades the `RwLockWriteGuard` to `RwLockReadGuard`
    #[inline(always)]
    pub fn downgrade(self) -> RwLockReadGuard<'rwlock, T> {
        let lock = unsafe {
            //  safety: memory layout guaranteed by #[repr(C)]
            NonNull::new_unchecked(self.0 as *mut _ as *mut RwLock<T>).as_ref()
        };
        core::mem::forget(self);

        unsafe { lock.rc.inner().store(1, Release); }

        RwLockReadGuard::new(unsafe { lock.data.get().as_ref_unchecked() })
    }

}


impl<T> core::ops::Deref for RwLockWriteGuard<'_, T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &Self::Target { self.0 }
}

impl<T> core::ops::DerefMut for RwLockWriteGuard<'_, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target { self.0 }
}


impl<T> Drop for RwLockWriteGuard<'_, T> {
    #[inline(always)]
    fn drop(&mut self) {
        let lock = unsafe {
            NonNull::new_unchecked(self.0 as *const _ as *mut RwLock<T>).as_ref()
        };

        debug_assert!(lock.rc.state_raw(Acquire) < 0);
        lock.rc.deactivate_writer();
    }
}

impl<T: Debug> Debug for RwLockWriteGuard<'_, T> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self.0, f)
    }
}


impl<T: Display> Display for RwLockWriteGuard<'_, T> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(self.0, f)
    }
}
