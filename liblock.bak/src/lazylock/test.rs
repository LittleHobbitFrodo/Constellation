use core::sync::atomic::{AtomicUsize, Ordering::{self, Acquire, Relaxed}};

use crate::{LazyLock, helpers::Status};



/*#[test]
fn init_access() {

    let call_count = AtomicUsize::new(0);

    let lock = LazyLock::new(|| {

        let ret = 66usize + call_count.load(Acquire);
        _ = call_count.fetch_add(1, Acquire);

        ret
    });
    assert!(matches!(LazyLock::get_status(&lock, Relaxed), Status::Uninit));

    assert!(*lock == 66);
    assert!(matches!(LazyLock::get_status(&lock, Relaxed), Status::Complete));

    assert!(*lock == 66);

}*/
