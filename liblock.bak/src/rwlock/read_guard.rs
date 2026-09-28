use core::{fmt::{Debug, Display}, ptr::NonNull, sync::atomic::Ordering::{self, Acquire, Relaxed}};
use  crate::{RwLock, RwLockWriteGuard, helpers::RwRc};


pub struct RwLockReadGuard<'rwlock, T>(&'rwlock T)
where T: 'rwlock;

impl<'rwlock, T> RwLockReadGuard<'rwlock, T> {


    pub(super) const fn new(data: &'rwlock T) -> Self {
        Self(data)
    }


    /// Leaks the `RwLockReadGuard` and returns reference to the shared data
    ///
    /// Note that this function locks the associated `RwLock` in read-only mode
    #[inline(always)]
    pub fn leak<'l>(guard: Self) -> &'l T
    where 'rwlock: 'l {
        let r = guard.0;
        core::mem::forget(guard);
        r
    }


    /// Converts the inner reference to a reference to the `RwLock`
    /// - Memory layout guaranteed by `#[repr(C)]`
    fn get_lock_ref(&self) -> &'rwlock RwLock<T> {
        unsafe {
            //  safety: memory layout guaranteed by #[repr(C)]
            NonNull::new_unchecked(self.0 as *const _  as *mut RwLock<T>).as_ref()
        }
    }


    /// Upgrades this `RwLockReadGuard` to `RwLockWriteGuard` if it can acquire exclusive
    /// access to the shared data, otherwise gives back the ownership
    #[inline]
    pub fn try_upgrade(self) -> Result<RwLockWriteGuard<'rwlock, T>, Self> {
        let lock = self.get_lock_ref();

        let inner = unsafe { lock.rc.inner() };

        match inner.compare_exchange(1, RwRc::WRITER_INDEX, Acquire, Relaxed) {
            Ok(_) => {
                let data = unsafe {
                    NonNull::new_unchecked(self.0 as *const _ as *mut T).as_mut()
                };
                core::mem::forget(self);
                Ok(RwLockWriteGuard::new(data))
            },
            Err(_) => Err(self)
        }
    }


    /// `upgrade()` spins until it can obtain exclusive access to the shared
    /// data and upgrades the `RwLockReadGuard` to `RwLockWriteGuard`
    pub fn upgrade(self) -> RwLockWriteGuard<'rwlock, T> {
        let lock = self.get_lock_ref();

        core::mem::forget(self);

        let inner = unsafe { lock.rc.inner() };

        while let Err(_) = inner.compare_exchange(1, RwRc::WRITER_INDEX, Acquire, Relaxed) {
            core::hint::spin_loop();
        }

        RwLockWriteGuard::new(unsafe { lock.data.get().as_mut_unchecked() })
    }

}


unsafe impl<T: Send> Send for RwLockReadGuard<'_, T> {}
unsafe impl<T: Send + Sync> Sync for RwLockReadGuard<'_, T> {}

impl<T> core::ops::Deref for RwLockReadGuard<'_, T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &Self::Target { self.0 }
}


impl<T> Drop for RwLockReadGuard<'_, T> {
    fn drop(&mut self) {
        let lock = self.get_lock_ref();
        debug_assert!(lock.rc.state_raw(Acquire) > 0);

        unsafe {
            lock.rc.remove_reader_unchecked();
        }
    }
}

impl<'rwlock, T> Clone for RwLockReadGuard<'rwlock, T> {
    fn clone(&self) -> Self {
        let lock = self.get_lock_ref();

        unsafe {
            //  safety: At least one read guard exists
            lock.rc.add_reader_unchecked();
        }

        RwLockReadGuard(self.0)
    }
}


impl<T: Debug> Debug for RwLockReadGuard<'_, T> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self.0, f)
    }
}

impl<T: Display> Display for RwLockReadGuard<'_, T> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(self.0, f)
    }
}
