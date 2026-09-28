use core::{marker::PhantomData, sync::atomic::{AtomicPtr, AtomicUsize, Ordering::{self, AcqRel}}};



/// Atomic pointer to `fn() -> T`
pub struct AtomicFn<T> {
    ptr: AtomicPtr<()>,
    _marker: PhantomData<fn() -> T>,
}


impl<T> AtomicFn<T> {

    /// Transmutes the pointer into `Option<fn() -> T>`
    const fn transmute_fn(f: *mut ()) -> Option<fn() -> T> {
        unsafe { core::mem::transmute_copy(&f) }
    }

    /// Transmutes the fn into a raw pointer
    const fn transmute_ptr(f: Option<fn() -> T>) -> *mut () {
        unsafe { core::mem::transmute_copy(&f) }
    }

    pub const fn new(f: Option<fn() -> T>) -> Self {
        Self {
            ptr: AtomicPtr::new(Self::transmute_ptr(f)),
            _marker: PhantomData
        }
    }

    /// Loads the pointer
    pub fn load(&self, order: Ordering) -> Option<fn() -> T> {
        Self::transmute_fn(self.ptr.load(order))
    }

    /// Stores the pointer
    pub fn store(&self, order: Ordering, f: Option<fn() -> T>) {
        self.ptr.store(Self::transmute_ptr(f), order);
    }

    pub fn swap(&self, new: Option<fn() -> T>, order: Ordering) -> Option<fn() -> T> {
        let ptr: *mut () = unsafe {
            core::mem::transmute_copy(&new)
        };
        debug_assert!(match new {
            Some(ptr) => ptr as *mut () as usize,
            None => 0,
        } == ptr as usize);
        unsafe {
            core::mem::transmute(self.ptr.swap(ptr, order))
        }
    }

    /// Same as `AtomicPtr::compare_exchange()`
    pub fn compare_exchange(&self, current: Option<fn() -> T>, new: Option<fn() -> T>, success: Ordering, failure: Ordering) -> Result<Option<fn() -> T>, Option<fn() -> T>> {
        let current = Self::transmute_ptr(current);
        let new = Self::transmute_ptr(new);
        match self.ptr.compare_exchange(current, new, success, failure) {
            Ok(ptr) => Ok(Self::transmute_fn(ptr)),
            Err(e) => Err(Self::transmute_fn(e))
        }
    }

}



#[test]
fn swap() {

    fn x() -> usize {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        COUNTER.fetch_add(1, AcqRel)
    }

    let x_ptr: fn() -> usize = x;

    let f = AtomicFn::new(Some(x));

    let swapped = f.swap(None, AcqRel);

    assert!(swapped.map(|ptr| ptr as usize ).unwrap_or(0) == x_ptr as usize);

    let swapped2 = f.swap(Some(x), AcqRel);

    assert!(matches!(swapped2, None));

}
