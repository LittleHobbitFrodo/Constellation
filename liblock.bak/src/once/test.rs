use core::sync::atomic::Ordering;

use crate::{Once, helpers::Status};



#[test]
fn init() {

    let once: Once<usize> = Once::new();

    assert!(matches!(once.get_status(Ordering::Acquire), Status::Uninit));
    assert!(matches!(once.get(), None));

    //  initialize
    assert!(*once.call_once(|| 420 ) == 420);


    assert!(matches!(once.get(), Some(420)));
    assert!(matches!(once.get_status(Ordering::Acquire), Status::Complete));

}

#[test]
fn double_init() {

    let once: Once<usize> = Once::new();

    assert!(matches!(once.get_status(Ordering::Acquire), Status::Uninit));

    //  initialize
    assert!(*once.call_once(|| 420 ) == 420);


    assert!(matches!(once.get_status(Ordering::Acquire), Status::Complete));


    assert!(*once.call_once(|| 67 ) == 420);

}


#[test]
fn reinit() {

    let once: Once<usize> = Once::new();

    assert!(matches!(once.get_status(Ordering::Acquire), Status::Uninit));

    //  initialize
    assert!(*once.call_once(|| 420 ) == 420);
    assert!(matches!(once.get_status(Ordering::Acquire), Status::Complete));


    //  deinitialize
    unsafe {
        once.force_status(Status::Uninit, Ordering::Release);
    }
    assert!(matches!(once.get_status(Ordering::Acquire), Status::Uninit));

    //  reinitialize
    assert!(*once.call_once(|| 67 ) == 67);
    assert!(*once.get().unwrap() == 67);

}
