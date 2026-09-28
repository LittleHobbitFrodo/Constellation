

use core::sync::atomic::{AtomicIsize, Ordering::{self, Relaxed, Release, Acquire}};


/// Atomically counts `n` readers **XOR** one writer
/// - This is not safe abstraction, but rather an helper struct for synchronization primitives like `RwLock`
///
/// # Implementation
///
///
/// `RwRc` atomically counts references using `AtomicIsize` like this:
/// - Positive value indicates reader count
/// - Negative values indicates an active writer
///   - The counter is set to `isize::MIN`, therefore any faulty call of `add_reader_unchecked()` will not mess with the counter
///   - Also any faulty call of `remove_reader_unchecked()` will panic with overflow
#[repr(transparent)]
pub struct RwRc(AtomicIsize);


impl RwRc {

    /// The specific (negative) number indicating writer presence
    pub const WRITER_INDEX: isize = isize::MIN;

    /// Constructs new `RwRc` with the count of `0` readers and `0` writers
    pub const fn new() -> Self { Self(AtomicIsize::new(0)) }

    /// Constructs new `RwRc` with one writer
    pub const fn new_writing() -> Self { Self(AtomicIsize::new(Self::WRITER_INDEX)) }

    /// Constructs new `RwRc` with `n` readers
    pub const fn new_reading(n: usize) -> Self { Self(AtomicIsize::new(n as isize)) }



    /// Indicates whether the counter has a writer
    /// - uses the `Relaxed` ordering
    ///   - Use the `state()` functions to change the ordering
    #[inline]
    pub fn has_writer(&self) -> bool { self.0.load(Relaxed) < 0 }

    /// Returns the count of all readers it a writer is not present
    /// - Uses the `Relaxed` ordering
    ///   - Use the `state()` function to change the ordering
    #[inline]
    pub fn reader_count(&self) -> Option<usize> {
        let state = self.0.load(Relaxed);
        match state {
            0.. => Some(state as usize),
            _ => None, //   writer is present
        }
    }

    /// Returns the reader count or `None` if writer is active
    /// - Lets the user decide which `Ordering` to use
    #[inline]
    pub fn state(&self, order: Ordering) -> Option<usize> {
        let state = self.0.load(order);
        match state {
            0.. => Some(state as usize),
            _ => None, //   writer is present
        }
    }

    /// Returns the underlying counter state
    /// - Read the `RwRc` doc
    #[inline]
    pub fn state_raw(&self, order: Ordering) -> isize { self.0.load(order) }


    /// Spins until a writer is deactivated and adds one reader
    pub fn add_reader(&self) {
        loop {
            let state = self.0.load(Acquire);

            if state >= 0 {  //  writer is not active
                let new = state.checked_add(1).expect("reader count overflow");
                if let Ok(_) = self.0.compare_exchange(state, new, Acquire, Relaxed) {
                    return
                }
            }

            core::hint::spin_loop();
        }
    }

    /// Tries to add a reader
    /// - Returns `Err` if a writer is active
    pub fn try_add_reader(&self) -> Result<(), ()> {
        self.0.try_update(Release, Acquire, |x| {
            if x >= 0 {
                Some(x.checked_add(1).expect("reader count overflow"))
            } else {
                None
            }
        }).map(|_| ()).map_err(|_| ())
    }

    /// Adds one reader without checking whether a writer is active
    /// - Panics if a writer is registered in debug builds
    ///
    /// # Safety
    /// The caller is responsible to ensure that the counter is not owned by a writer before calling this function
    ///
    /// This function may cause undefined behaviour when a writer is registered
    /// ## Specific behaviour
    /// Since the internal value is se to `isize::MIN`, when a writer is active this action
    /// should have no effect (unless you call it `usize::MAX/2` times per one writer)
    /// - Therefore when a writer is active and we want to `remove_reader_unchecked()`, the inner value will overflow and the system will panic
    #[inline]
    pub unsafe fn add_reader_unchecked(&self) {
        _ = self.0.update(Release, Acquire, |x| {
            debug_assert!(x >= 0, "writer is present");

            //  overflow would cause UB
            x.checked_add(1).expect("reader count overflow")
        });
    }


    /// Removes one reader
    /// - Returns `Err` if writer is active or there are no readers
    pub fn remove_reader(&self) -> Result<(), ()> {
        loop {
            let state = self.0.load(Acquire);
            match state {
                1.. => {    //  0 = no readers
                    let new = state - 1;
                    if let Ok(_) = self.0.compare_exchange(state, new, Acquire, Relaxed) {
                        return Ok(())
                    }
                },
                ..1 => return Err(())
            }
        }
    }

    /// Removes one reader without checking if a reader is active
    /// - Panics on overflow (therefore when a writer is present)
    #[inline]
    pub unsafe fn remove_reader_unchecked(&self) {
        self.0.update(Release, Acquire, |x| {
            debug_assert!(x >= 0, "writer is present");

            //  optimized builds could fall into UB if not checked
            x.checked_sub(1).expect("writer registered")
        });
    }


    /// Activates a writer, spins until there are no readers
    pub fn activate_writer(&self) {
        loop {
            if let Ok(_) = self.0.compare_exchange(0, Self::WRITER_INDEX, Acquire, Relaxed) {
                core::hint::cold_path();
                break
            }
        }
    }

    /// Returns the inner atomic integer
    #[inline(always)]
    pub(crate) unsafe fn inner(&self) -> &AtomicIsize { &self.0 }

    /// Adds a writer if there are no readers and no writer
    #[inline]
    pub fn try_activate_writer(&self) -> Result<(), ()> {
        self.0.compare_exchange(0, Self::WRITER_INDEX, Acquire, Relaxed).map(|_| ()).map_err(|_| ())
    }

    /// Removes the writer
    /// - No-op if writer is not present
    #[inline]
    pub fn deactivate_writer(&self) {
        if let Ok(_) = self.0.compare_exchange(Self::WRITER_INDEX, 0, Acquire, Relaxed) {
            core::hint::cold_path();
        }
    }
}


impl core::fmt::Debug for RwRc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.state(Relaxed) {
            Some(readers) => write!(f, "RwRc {{ readers: {readers} }}"),
            None => write!(f, "RwRc {{ writer }}"),
        }
    }
}
