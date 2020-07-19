pub mod rwlock {
    use std::sync::{Arc, RwLock};
    use std::thread;

    #[test]
    fn test() {
        let lock = RwLock::new(5);

        // many reader locks can be held at once
        {
            let r1 = lock.read().unwrap();
            let r2 = lock.read().unwrap();
            assert_eq!(*r1, 5);
            assert_eq!(*r2, 5);
        } // read locks are dropped at this point

        // only one write lock may be held, however
        {
            let mut w = lock.write().unwrap();
            *w += 1;
            assert_eq!(*w, 6);
        }
    }

    #[test]
    fn test2() {
        let lock = Arc::new(RwLock::new(1));
        let c_lock = lock.clone();

        let n = lock.read().unwrap();
        assert_eq!(*n, 1);

        thread::spawn(move || {
            let r = c_lock.read();
            assert!(r.is_ok());
        })
        .join()
        .unwrap();
    }
}
