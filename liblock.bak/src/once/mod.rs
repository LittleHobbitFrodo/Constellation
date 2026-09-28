use core::{cell::UnsafeCell, debug_assert_matches, mem::{ManuallyDrop, MaybeUninit}, ptr::{NonNull, drop_in_place}, sync::atomic::Ordering::{self, Acquire, Relaxed, Release}};

use crate::helpers::{AtomicStatus, Status};


#[cfg(test)]
mod test;


/// A primitive that provides lazy one-time initialization
#[repr(C)]
pub struct Once<T: Sized> {
    data: UnsafeCell<MaybeUninit<T>>,
    status: AtomicStatus
}

unsafe impl<T: Sized> Send for Once<T> {}
unsafe impl<T: Sized> Sync for Once<T> {}

impl<T: Sized> Once<T> {

    /// Constructs new `Once`
    pub const fn new() -> Self {
        Self {
            data: UnsafeCell::new(MaybeUninit::uninit()),
            status: AtomicStatus::new(crate::helpers::Status::Uninit)
        }
    }

    /// Returns the initialization `Status` of this `Once`
    pub fn get_status(&self, order: Ordering) -> Status {
        self.status.load(order)
    }

    /// Forces the initialization `Status` of this `Once`
    ///
    /// This can be useful when forcing the status to `Uninit` for re-initialization
    /// - Note that the current value will be treated as uninitialized
    /// memory so its destructor will never run
    ///
    /// # Safety
    /// Rewriting the status of an uninitialized `Once` will:
    /// - Access of uninitialized memory, thus violating rust's
    /// memory initialization rule and falling into undefined
    /// behaviour if forced to `Complete`
    /// - Endless spinning on access if forced to `Running`
    pub unsafe fn force_status(&self, status: Status, order: Ordering) {
        self.status.store(status, order);
    }


    /// Initializes the data if it has not been initialized yet and returns an reference to the data
    pub fn call_once<F: FnOnce() -> T>(&self, init: F) -> &T {

        match self.status.load(Ordering::Acquire) {
            Status::Uninit => match self.status.update_if(Status::Uninit, Status::Running, Acquire, Relaxed) {
                Ok(_) => {  //  initialize
                    self.initialize(init());
                    self.status.store(Status::Complete, Release);
                    unsafe { self.get_unchecked() }
                },
                Err(_) => { //  initialization is already in progress
                    self.wait()
                }
            },
            Status::Running => self.wait(),
            Status::Complete => unsafe { self.get_unchecked() }
        }
    }


    /// Spins until the value is initialized
    pub fn wait(&self) -> &T {
        loop {
            match self.status.load(Ordering::Acquire) {
                Status::Complete => break unsafe { self.get_unchecked() },
                _ => core::hint::spin_loop(),
            }
        }
    }

    /// Line `get()`, but spins if the initialization has begun, returns `None` if not
    pub fn poll(&self) -> Option<&T> {

        loop {
            match self.status.load(Ordering::Acquire) {
                Status::Uninit => break None,
                Status::Running => core::hint::spin_loop(),
                Status::Complete => break Some(unsafe { self.get_unchecked() })
            }
        }
    }


    /// Returns a `NonNull` pointer to the data
    pub fn as_ptr(&self) -> NonNull<T> {
        unsafe {
            NonNull::new_unchecked(self.data.get().cast())
        }
    }

    /// Initializes the data
    #[inline]
    fn initialize(&self, val: T) {
        unsafe {
            NonNull::new_unchecked(self.data.get()).cast::<T>().write(val);
        }
    }

    /// Returns reference to the underlying data without checking if the data has been initialized
    #[inline]
    pub unsafe fn get_unchecked(&self) -> &T {
        debug_assert_matches!(self.status.load(Ordering::Acquire), Status::Complete);
        unsafe {
            &*(*self.data.get()).as_ptr()
        }
    }


    /// Returns reference to the underlying data if it has been initialized
    #[inline]
    pub fn get(&self) -> Option<&T> {
        if let Status::Complete = self.status.load(Ordering::Acquire) {
            Some(unsafe { self.get_unchecked() })
        } else {
            None
        }
    }


    /// Returns a mutable reference to the underlying data
    ///
    /// Safety: Given that the function takes mutable reference to `self`, this is safe
    ///
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if let Status::Complete = self.status.load(Ordering::Acquire) {
            Some(unsafe { self.get_mut_unchecked() })
        } else {
            None
        }
    }


    /// Returns a mutable reference to the underlying data without checking if it is actually initialized
    pub unsafe fn get_mut_unchecked(&mut self) -> &mut T {
        debug_assert_matches!(self.status.load(Ordering::Acquire), Status::Complete);
        unsafe {
            self.as_ptr().as_mut()
        }
    }

    /// Returns the inner value if the `Once` has been initialized
    ///
    /// Since this function requires ownership, this action requires
    /// no synchronization overhead and thus is zero-cost
    pub fn try_into_inner(self) -> Option<T> {
        let me = ManuallyDrop::new(self);
        if let Status::Complete = me.status.load(Ordering::Acquire) {
            unsafe {
                Some(me.as_ptr().read())
            }
        } else {
            None
        }
    }


    /// Returns the inner value without checking whether the `Once` has been initialized
    ///
    /// Since this function requires ownership, this action requires
    /// no synchronization overhead and thus is zero-cost
    ///
    /// # Safety
    /// This function returns uninitialized object if the `Once` is not initialized
    /// - Use the `is_initialized()` function to check whether the `Once` is initialized before proceeding to call this function
    #[inline]
    pub unsafe fn into_inner_unchecked(self) -> T {
        debug_assert_matches!(self.status.load(Ordering::Acquire), Status::Complete);
        let me = ManuallyDrop::new(self);
        unsafe {
            me.as_ptr().read()
        }
    }

    /// Indicates whether the `Once` is initialized
    #[inline]
    pub fn is_initialized(&self) -> bool {
        matches!(self.status.load(Ordering::Acquire), Status::Complete)
    }


}


impl<T: Sized> From<T> for Once<T> {
    fn from(value: T) -> Self {
        Self {
            data: UnsafeCell::new(MaybeUninit::new(value)),
            status: AtomicStatus::new(Status::Complete)
        }
    }
}

impl<T: Sized> Drop for Once<T> {
    fn drop(&mut self) {
        if let Status::Complete = self.status.load(Ordering::Acquire) {
            unsafe {
                drop_in_place(self.data.get().cast::<T>());
            }
        }
    }
}
