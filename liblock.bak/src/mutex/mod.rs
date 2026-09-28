use core::{cell::UnsafeCell, fmt::{Debug, Display}, marker::PhantomData, mem::ManuallyDrop, ptr::NonNull, slice::GetDisjointMutError};

use crate::helpers::Lock;


#[cfg(test)]
mod tests;



/// Provides mutually exclusive access to data shared between threads
#[repr(C)]
pub struct Mutex<T: Sized> {
    data: UnsafeCell<T>,
    lock: Lock,
}

unsafe impl<T: Sized> Send for Mutex<T> {}
unsafe impl<T: Sized> Sync for Mutex<T> {}

impl<T: Sized> Mutex<T> {

    /// Constructs new `Mutex`
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
            lock: Lock::new(),
        }
    }

    #[cfg(test)]
    /// Returns the underlying `Lock`
    pub(crate) fn get_lock(&self) -> &Lock { &self.lock }


    /// Blocks the current thread until able to lock the `Mutex`
    pub fn lock<'l>(&self) -> MutexGuard<'l, T> {
        self.lock.lock();
        MutexGuard {
            data: unsafe { NonNull::new_unchecked(self.data.get()) },
            _marker: PhantomData
        }
    }


    /// Tries to lock the `Mutex`, returns `Err` if it is already locked
    pub fn try_lock<'l>(&self) -> Result<MutexGuard<'l, T>, ()> {
        self.lock.try_lock()?;
        Ok(MutexGuard {
            data: unsafe { NonNull::new_unchecked(self.data.get()) },
            _marker: PhantomData
        })
    }


    /// Indicates whether the `Mutex` is locked
    #[inline(always)]
    pub fn is_locked(&self) -> bool { self.lock.is_locked() }

    /// Forcibly unlocks the `Mutex`
    #[inline(always)]
    pub unsafe fn force_unlock(&self) { self.lock.unlock() }

    /// Returns mutable reference to the underlying data
    ///
    /// Since this function demands mutable borrow, this is safe
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        unsafe {
            NonNull::new_unchecked(self.data.get()).as_mut()
        }
    }


}

impl<T: Sized> Debug for Mutex<T>
where T: Debug {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.try_lock() {
            Ok(guard) => write!(f, "Mutex {{ data: {:?} }}", &*guard),
            Err(_) => write!(f, "Mutex <locked>"),
        }
    }
}

impl<T: Sized> Default for Mutex<T>
where T: Default {
    #[inline(always)]
    fn default() -> Self { Self::new(T::default()) }
}



impl<T: Sized> From<T> for Mutex<T> {
    #[inline(always)]
    fn from(value: T) -> Self { Self::new(value) }
}





/// A guard providing mutable access to data
pub struct MutexGuard<'l, T: Sized + 'l> {
    data: NonNull<T>,
    _marker: PhantomData<&'l mut T>
}

unsafe impl<'l, T: Sized + 'l> Send for MutexGuard<'l, T> {}
unsafe impl<'l, T: Sized + 'l> Sync for MutexGuard<'l, T> {}

impl<'l, T: Sized> MutexGuard<'_, T> {

    /// Leaks the `MutexGuard` and keeps the `Mutex` locked
    pub fn leak<'a>(this: Self) -> &'a mut T
    where 'l: 'a {
        let mut me = ManuallyDrop::new(this);
        unsafe { me.data.as_mut() }
    }

}


impl<T: Sized> core::ops::Deref for MutexGuard<'_, T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target { unsafe { self.data.as_ref() } }
}

impl<T: Sized> core::ops::DerefMut for MutexGuard<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { unsafe { self.data.as_mut() } }
}

impl<T: Sized + Display> Display for MutexGuard<'_, T> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&**self, f)
    }
}

impl<T: Sized + Debug> Debug for MutexGuard<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&**self, f)
    }
}


impl<T: Sized> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        unsafe {
            self.data.add(1).cast::<Lock>().as_ref().unlock();
        }
    }
}
