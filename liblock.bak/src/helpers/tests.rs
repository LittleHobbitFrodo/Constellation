use core::assert_matches;


use super::RwRc;
use core::sync::atomic::Ordering::{self, Relaxed};

use super::*;

mod lock {
    use super::*;

    #[test]
    fn lock() {

        let lock = Lock::new();
        assert!(lock.is_locked() == false);

        lock.lock();
        assert!(lock.is_locked() == true);


        lock.unlock();
        assert!(lock.is_locked() == false);

    }
}



mod status {
    use super::*;

    #[test]
    fn init() {

        let status = AtomicStatus::new(Status::Complete);

        assert_matches!(status.load(Ordering::Acquire), Status::Complete);
    }


    #[test]
    fn update() {

        let status = AtomicStatus::new(Status::Uninit);

        //  updatig if current = Running => failure
        let update = status.update_if(Status::Running, Status::Complete, Ordering::Acquire, Ordering::Relaxed);
        assert!(matches!(update, Err(_)));

        //  status did not change
        assert!(matches!(status.load(Ordering::Acquire), Status::Uninit));


        //  successful update
        let update = status.update_if(Status::Uninit, Status::Running, Ordering::Acquire, Ordering::Relaxed);
        assert!(matches!(update, Ok(Status::Uninit)));
        assert!(matches!(status.load(Ordering::Acquire), Status::Running));


        //  updating if status = Uninit => failure
        let update = status.update_if(Status::Uninit, Status::Complete, Ordering::Acquire, Ordering::Relaxed);
        assert!(matches!(update, Err(_)));

        assert!(matches!(status.load(Ordering::Acquire), Status::Running));


    }
}


mod rwrc {
    use super::*;

    #[test]
    fn init() {

        let zero = RwRc::new();
        assert!(zero.state_raw(Relaxed) == 0);

        assert!(matches!(zero.try_add_reader(), Ok(_)));
        assert!(zero.state_raw(Relaxed) == 1);

        assert!(matches!(zero.remove_reader(), Ok(_)));
        assert!(zero.state_raw(Relaxed) == 0);



        let writing = RwRc::new_writing();
        assert!(writing.state_raw(Relaxed) == RwRc::WRITER_INDEX);
        assert!(matches!(writing.try_activate_writer(), Err(_)));

        writing.deactivate_writer();

        assert!(matches!(writing.try_activate_writer(), Ok(_)));
        assert!(writing.state_raw(Relaxed) == RwRc::WRITER_INDEX);

        writing.deactivate_writer();

        assert!(matches!(writing.try_add_reader(), Ok(_)));
        assert!(writing.state_raw(Relaxed) == 1);


        let readers = RwRc::new_reading(66);
        assert!(readers.state_raw(Relaxed) == 66);
        assert!(matches!(readers.try_add_reader(), Ok(_)));
        assert!(readers.state_raw(Relaxed) == 67);

        assert!(matches!(readers.try_activate_writer(), Err(_)));
        assert!(readers.state_raw(Relaxed) == 67);

        assert!(matches!(readers.remove_reader(), Ok(_)));
        assert!(readers.state_raw(Relaxed) == 66);

    }


}
