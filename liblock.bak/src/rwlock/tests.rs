use core::sync::atomic::Ordering::{self, Acquire, Relaxed};

use crate::{RwLock, RwLockReadGuard, RwLockWriteGuard, helpers::RwRc};



#[test]
fn read_write() {

    let lock = RwLock::new(66usize);
    assert!(lock.rc.state_raw(Relaxed) == 0);
    assert!(*unsafe { lock.data.get().as_ref_unchecked() } == 66);

    let reader = lock.read();
    assert!(lock.rc.state_raw(Relaxed) == 1);
    assert!(*reader == 66);

    {
        let reader2 = lock.try_read()
            .expect("failed to acquire second reader");

        assert!(*reader2 == 66);
        assert!(lock.rc.state_raw(Relaxed) == 2);
    }

    assert!(lock.rc.state_raw(Relaxed) == 1);

    assert!(matches!(lock.try_write(), None));

    drop(reader);
    assert!(lock.rc.state_raw(Relaxed) == 0);


    let mut writer = lock.write();
    assert!(lock.rc.state_raw(Relaxed) == RwRc::WRITER_INDEX);
    assert!(*unsafe { lock.data.get().as_ref_unchecked() } == 66);

    assert!(matches!(lock.try_write(), None));


    *writer = 42;
    assert!(*writer == 42);

    assert!(matches!(lock.try_read(), None));
    drop(writer);
    assert!(lock.rc.state_raw(Relaxed) == 0);

}


#[test]
fn try_upgrade() {

    let lock = RwLock::new(66usize);

    let reader = lock.read();
    assert!(lock.rc.state_raw(Relaxed) == 1);

    let mut writer = match reader.try_upgrade() {
        Ok(w) => w,
        Err(_) => panic!("failed to upgrade sole reader"),
    };

    assert!(lock.rc.state_raw(Relaxed) == RwRc::WRITER_INDEX);
    assert!(*writer == 66);

    *writer = 42;
    assert!(*writer == 42);

    drop(writer);
    assert!(lock.rc.state_raw(Relaxed) == 0);

    let reader = lock.read();
    let reader2 = lock.read();

    assert!(matches!(reader.try_upgrade(), Err(_)),
        "reader upgraded when multiple of them exists");

    assert!(lock.rc.state_raw(Relaxed) == 1);
    assert!(*reader2 == 42);

    let mut writer = match reader2.try_upgrade() {
        Ok(w) => w,
        Err(_) => panic!("failed to upgrade sole reader"),
    };

    assert!(lock.rc.state_raw(Relaxed) == RwRc::WRITER_INDEX);
    *writer += 1;
    assert!(*writer == 43);

}


#[test]
fn downgrade() {

    let lock = RwLock::new(66usize);

    let mut writer = lock.try_write()
        .expect("failed to get writer");

    *writer -= 1;

    assert!(lock.rc.state_raw(Relaxed) == RwRc::WRITER_INDEX);

    let reader = writer.downgrade();
    assert!(lock.rc.state_raw(Relaxed) == 1);
    assert!(*reader == 65);
}



#[test]
fn leak() {

    let lock1 = RwLock::new(66usize);

    let reader = lock1.try_read()
        .expect("failed to get reader");
    assert!(lock1.rc.state_raw(Acquire) == 1);

    let leaked1 = RwLockReadGuard::leak(reader);
    assert!(lock1.rc.state_raw(Acquire) == 1);
    assert!(*leaked1 == 66);
    assert!(matches!(lock1.try_write(), None));



    let lock2 = RwLock::new(420usize);

    let writer = lock2.try_write()
        .expect("failed to acquire writer");
    assert!(lock2.rc.state_raw(Relaxed) == RwRc::WRITER_INDEX);


    let leaked2 = RwLockWriteGuard::leak(writer);
    assert!(lock2.rc.state_raw(Relaxed) == RwRc::WRITER_INDEX);
    assert!(*leaked2 == 420);

}
