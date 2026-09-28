
#[cfg(target_has_atomic = "8")]
use core::sync::atomic::AtomicU8;


#[cfg(not(target_has_atomic = "8"))]
use core::sync::atomic::AtomicU32;

use core::sync::atomic::Ordering;



/// Thread safe status for sync primitives like `Once` or `lazy`
#[repr(transparent)]
pub struct AtomicStatus(AtomicStatusInternal);


#[cfg(target_has_atomic = "8")]
type AtomicStatusInternal = AtomicU8;
#[cfg(not(target_has_atomic = "8"))]
type AtomicStatusInternal = AtomicU32;

#[cfg(target_has_atomic = "8")]
type StatusInternal = u8;
#[cfg(not(target_has_atomic = "8"))]
type StatusInternal = u32;


impl AtomicStatus {

    /// Constructs new `AtomicStatus`
    pub const fn new(status: Status) -> Self {
        Self(AtomicStatusInternal::new(status as StatusInternal))
    }

    #[inline(always)]
    pub fn load(&self, order: Ordering) -> Status {
        unsafe {
            core::mem::transmute(self.0.load(order))
        }
    }

    #[inline(always)]
    pub fn store(&self, status: Status, order: Ordering) {
        self.0.store(status as StatusInternal, order);
    }


    /// Updates the `AtomicStatus` if its current value matches `current`
    /// - See `AtomicU8::compare_exchange`
    pub fn update_if(&self, current: Status, new: Status, success: Ordering, failure: Ordering) -> Result<Status, Status> {
        unsafe {
            core::mem::transmute(self.0.compare_exchange(current as StatusInternal, new as StatusInternal, success, failure))
        }
    }


}

#[cfg(target_has_atomic = "8")]
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Status {
    Uninit = 0,
    Running = 1,
    Complete = 2,
}

#[cfg(not(target_has_atomic = "8"))]
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Status {
    Uninit = 0,
    Running = 1,
    Complete = 2,
}

//#[cfg(debug_assertions)]
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", match *self {
            Self::Uninit => "Status::Uninit",
            Self::Running => "Status::Running",
            Self::Complete => "Status::Complete"
        })
    }
}
