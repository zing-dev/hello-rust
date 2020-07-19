pub mod arc {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test() {
        let five = Arc::new(5);

        for _ in 0..10 {
            let five = Arc::clone(&five);

            thread::spawn(move || {
                println!("{:?}", five);
                assert_eq!(*five, 5)
            })
            .join()
            .unwrap();
        }
    }

    #[test]
    fn test2() {
        let val = Arc::new(AtomicUsize::new(5));
        for _ in 0..10 {
            let val = Arc::clone(&val);
            let v = val.fetch_add(1, Ordering::SeqCst);
            println!("{:?}", v);
            // thread::spawn(move || {
            //     let v = val.fetch_add(1, Ordering::SeqCst);
            //     println!("{:?}", v);
            // });
        }
    }
}
