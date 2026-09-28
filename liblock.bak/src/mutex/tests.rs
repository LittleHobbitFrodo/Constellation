use core::ptr::NonNull;

use crate::{helpers::Lock, mutex::Mutex};




#[test]
fn mutex_new() {

    let mutex = Mutex::new(67usize);

    unsafe {    //  inspect the memory layout
        let ptr = NonNull::from(&mutex).cast::<usize>();
        assert!(ptr.read() == 67);
        assert!(ptr.add(1).cast::<Lock>().as_ref().is_locked() == false);
    }

}


#[test]
fn lock() {

    let mutex = Mutex::new(420usize);
    assert!(mutex.is_locked() != true);


    let guard = mutex.lock();
    assert!(mutex.is_locked());
    assert!(*guard == 420);


    assert!(matches!(mutex.try_lock(), Err(_)));

    //  manually unlock the mutex
    drop(guard);

    assert!(mutex.is_locked() != true);

    assert!(matches!(mutex.try_lock(), Ok(_)));
}


#[test]
fn force_unlock() {


    let mutex = Mutex::new(66usize);
    assert!(mutex.get_lock().is_locked() != true);

    let guard = mutex.lock();
    assert!(mutex.get_lock().is_locked());
    assert!(*guard == 66);

    unsafe { mutex.force_unlock() };

    assert!(mutex.get_lock().is_locked() == false);

    let mut guard2 = mutex.try_lock()
        .expect("failed to lock the unlocked mutex");
    assert!(mutex.is_locked());
    assert!(*guard == *guard2);

    *guard2 = 420;

    assert!(*guard2 == 420);



    drop(guard);

    assert!(mutex.is_locked() == false);

    drop(guard2);

    assert!(mutex.is_locked() == false);
}
