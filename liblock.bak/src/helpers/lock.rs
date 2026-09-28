
#[cfg(target_has_atomic = "8")]
use core::sync::atomic::AtomicBool;

#[cfg(not(target_has_atomic = "32"))]
use core::sync::atomic::AtomicU32;


use core::sync::atomic::Ordering::{self, Relaxed, Release, Acquire};


/// A simple atomic locking mechanism
///
/// > Note: This is a raw API meant to be abstracted away from the user, no safety guarantees are held
#[repr(transparent)]
pub struct Lock(LockInternal);



#[cfg(target_has_atomic = "8")]
type LockInternal = AtomicBool;

#[cfg(target_has_atomic = "8")]
type LockPrimitive = bool;


#[cfg(not(target_has_atomic = "8"))]
type LockInternal = AtomicU32;

#[cfg(not(target_has_atomic = "8"))]
type LockPrimitive = u32;


impl Lock {

    /// Constructs new unlocked `Lock`
    pub const fn new() -> Self { Self(AtomicBool::new(Self::UNLOCKED)) }


    /// Indicates whether the `Lock` is locked
    /// - Uses the `Relaxed` ordering
    ///   - Use the `is_locked_ordered()` function to change the ordering
    #[inline]
    pub fn is_locked(&self) -> bool { self.0.load(Relaxed) == Self::LOCKED }


    /// Same as `is_locked()`, but lets the user choose atomic ordering
    #[inline]
    pub(crate) fn is_locked_ordered(&self, order: Ordering) -> LockPrimitive { self.0.load(order) }



    /// Blocks the current thread until able to lock the `Lock`
    pub fn lock(&self) {
        while let Err(_) = self.0.compare_exchange(Self::UNLOCKED, Self::LOCKED, Acquire, Relaxed) {
            core::hint::spin_loop();
        }
    }

    /// Tries to lock the `Lock`, returns `Err` if it is already locked
    #[inline]
    pub fn try_lock(&self) -> Result<(), ()> {
        match self.0.compare_exchange(Self::UNLOCKED, Self::LOCKED, Acquire, Relaxed) {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }

    /// Unlocks this `Lock`
    ///
    /// # Safety
    /// This function will certainly cause undefined behaviour if the `MutexGuard` is not held by the current thread
    #[inline]
    pub fn unlock(&self) { self.0.store(Self::UNLOCKED, Release); }

}


#[cfg(target_has_atomic = "8")]
impl Lock {
    pub(super) const LOCKED: bool = true;
    pub(super) const UNLOCKED: bool = false;
}

#[cfg(not(target_has_atomic = "8"))]
impl Lock {
    pub(super) const LOCKED: u32 = u32::MAX;
    pub(super) const UNLOCKED: u32 = 0;
}


#[cfg(debug_assertions)]
impl core::fmt::Debug for Lock {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Lock({})", if self.is_locked() {
            "locked"
        } else {
            "unlocked"
        })
    }
}
