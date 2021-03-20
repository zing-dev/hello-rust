pub mod quick_start {
    use std::borrow::{Borrow, BorrowMut};
    use std::thread::{current, Builder};
    use std::time::Duration;
    use std::{panic, thread};

    #[test]
    //Use of mutable static is unsafe and requires unsafe function or block [E0133]
    fn test() {
        static mut V: i32 = 0;
        fn unsafe_seq() -> i32 {
            unsafe {
                V += 1;
                V
            }
        }
        unsafe_seq();
        println!("{}", unsafe { V });
    }

    #[test]
    fn t() {
        static mut V: i32 = 0;
        fn unsafe_seq() -> i32 {
            unsafe {
                V += 1;
                V
            }
        }
        let child = thread::spawn(move || {
            for _ in 0..1000 {
                unsafe_seq();
                unsafe { println!("child: {}", V) }
            }
        });
        for _ in 0..1000 {
            unsafe_seq();
            unsafe { println!("main: {}", V) }
        }
        child.join().unwrap();
    }

    #[test]
    fn t2() {
        let mut v = vec![];
        for id in 0..5 {
            let child = thread::spawn(move || {
                println!("in child: {}", id);
            });
            v.push(child)
        }
        println!("in main: join before");
        for child in v {
            child.join().unwrap();
        }
        println!("in main: join after");
    }

    #[test]
    fn builder() {
        let mut v = vec![];
        for id in 0..5 {
            let thread_name = format!("child-{}", id);
            let size: usize = 3 * 1024;
            let builder = Builder::new().name(thread_name).stack_size(size);
            let child = builder
                .spawn(move || {
                    println!("in child: {}", id);
                    if id == 3 {
                        // panic::catch_unwind(|| {
                        //     panic!("oh no!");
                        // });
                        println!("in {} do sm", current().name().unwrap())
                    }
                })
                .unwrap();
            v.push(child);
        }
        for child in v {
            child.join().unwrap();
        }
    }

    #[test]
    fn local() {
        thread_local!(static FOO:std::cell::RefCell<u32> = std::cell::RefCell::new(1));
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 2;
        });
        println!("=> {:?}", FOO);
        thread::spawn(|| {
            FOO.with(|f| {
                assert_eq!(*f.borrow(), 1);
                *f.borrow_mut() = 3;
            });
        });
        FOO.with(|f| assert_eq!(*f.borrow(), 2));
        println!("=> {:?}", FOO);
    }

    #[test]
    fn park() {
        let handle = thread::Builder::new()
            .spawn(|| {
                println!("parking thread");
                thread::park();
                println!("thread unparked");
            })
            .unwrap();
        thread::sleep(Duration::from_millis(10));
        println!("unpark the thread");
        handle.thread().unpark();
        handle.join().unwrap();
    }

    pub mod mutex {
        use std::cell::RefCell;
        use std::rc::Rc;
        use std::sync::{Arc, Mutex};
        use std::thread;

        // #[test]
        // // error[E0382]: use of moved value: `s`
        // fn err1() {
        //     let mut s = String::from("hello");
        //     // move occurs because `s` has type `String`,
        //     // which does not implement the `Copy` trait
        //     for _ in 0..3 {
        //         thread::spawn(move || s.push_str("rust"));
        //     }
        // }

        // #[test]
        // // error[E0277]: `Rc<String>` cannot be sent between threads safely
        // // the trait `Send` is not implemented for `Rc<String>`
        // fn err2() {
        //     let mut s = Rc::new("hello".to_string());
        //     for _ in 0..3 {
        //         let mut s2 = s.clone();
        //         thread::spawn(move || {
        //             s2.push_str("rust");
        //         });
        //     }
        // }

        // #[test]
        // fn err3() {
        //     let s = Arc::new(String::from("hello"));
        //     for _ in 0..3 {
        //         // error[E0596]: cannot borrow data in an `Arc` as mutable
        //         // trait `DerefMut` is required to modify through a dereference,
        //         // but it is not implemented for `Arc<String>`
        //         let mut s2 = s.clone();
        //         thread::spawn(move || {
        //             s2.push_str("rust");
        //         });
        //     }
        // }

        // #[test]
        // // error[E0277]: `RefCell<String>` cannot be shared between threads safely
        // // the trait `Sync` is not implemented for `RefCell<String>`
        // fn err4() {
        //     let s = Arc::new(RefCell::new(String::from("hello")));
        //     for _ in 0..3 {
        //         let s2 = s.clone();
        //         thread::spawn(move || {
        //             let mut s2 = s.borrow_mut();
        //             s2.push_str("world");
        //         });
        //     }
        // }

        #[test]
        fn finally() {
            let s = Arc::new(Mutex::new(String::from("hello")));
            let mut v = vec![];
            for _ in 0..3 {
                let s2 = s.clone();
                let child = thread::spawn(move || {
                    s2.lock().unwrap().push_str("rust");
                });
                v.push(child);
            }
            for child in v {
                child.join().unwrap();
            }
            println!("{:?}", s);
        }

        #[test]
        fn poison() {
            let mtx = Arc::new(Mutex::new(1));
            let m2 = mtx.clone();
            let _ = thread::spawn(move || {
                let mut data = m2.lock().unwrap();
                *data = 2;
                panic!("fuck!")
            })
            .join();
            assert_eq!(mtx.is_poisoned(), true);
            match mtx.lock() {
                Ok(_) => unreachable!(),
                Err(e) => {
                    let data = e.get_ref();
                    println!("recovered: {}", data);
                }
            };
        }
    }

    pub mod rwlock {
        use std::ops::Deref;
        use std::sync::RwLock;

        #[test]
        fn test() {
            let lock = RwLock::new(1);
            {
                let r1 = lock.read().unwrap();
                let r2 = lock.read().unwrap();
                println!("{} {}", r1, r2)
            }
            {
                let mut w = lock.write().unwrap();
                *w += 10;
                println!("{} {}", *w, w.deref());
            }
        }
    }

    pub mod barrier {
        use std::sync::{Arc, Barrier};
        use std::thread;

        #[test]
        fn test() {
            let mut v = Vec::with_capacity(5);
            let b = Arc::new(Barrier::new(5));
            for _ in 0..5 {
                let b2 = b.clone();
                v.push(thread::spawn(move || {
                    println!("before wait");
                    b2.wait();
                    println!("after wait");
                }))
            }
            for h in v {
                h.join().unwrap();
            }
        }
    }

    pub mod cond {
        use std::sync::atomic::AtomicBool;
        use std::sync::{Arc, Condvar, Mutex};
        use std::thread;

        #[test]
        fn test() {
            let pair = Arc::new((Mutex::new(false), Condvar::new()));
            let p2 = pair.clone();
            thread::spawn(move || {
                let (ref lock, ref c) = &*p2;
                let mut started = lock.lock().unwrap();
                *started = true;
                c.notify_one()
            });
            let (lock, c) = &*pair;
            let mut started = lock.lock().unwrap();
            while !*started {
                println!("{}", started);
                started = c.wait(started).unwrap();
                println!("{}", started);
            }
        }

        #[test]
        fn test2() {
            let pair = Arc::new((Mutex::new(false), Condvar::new()));
            let pair2 = Arc::clone(&pair);

            thread::spawn(move || {
                let (lock, cvar) = &*pair2;
                let mut started = lock.lock().unwrap();
                *started = true;
                // We notify the condvar that the value has changed.
                cvar.notify_one();
            });

            // Wait for the thread to start up.
            let (lock, cvar) = &*pair;
            let mut started = lock.lock().unwrap();
            // As long as the value inside the `Mutex<bool>` is `false`, we wait.
            while !*started {
                println!("{}", started);
                started = cvar.wait(started).unwrap();
                println!("{}", started);
            }
        }
    }
}
