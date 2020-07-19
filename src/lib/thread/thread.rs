pub mod thread {
    use std::thread;
    use std::time::Duration;

    #[test]
    fn spawn() {
        let handler = thread::spawn(|| {
            let str = "hello world";
            println!("==> {}", str);
            str
        });
        println!("{}", handler.join().unwrap());
    }

    #[test]
    fn spawn2() {
        use std::sync::mpsc::channel;
        use std::thread;
        let (tx, rx) = channel();
        let sender = thread::spawn(move || {
            tx.send("Hello, thread".to_owned())
                .expect("Unable to send on channel");
        });
        let receiver = thread::spawn(move || {
            let value = rx.recv().expect("Unable to receive from channel");
            println!("{}", value);
        });
        sender.join().expect("The sender thread has panicked");
        receiver.join().expect("The receiver thread has panicked");
    }

    #[test]
    fn spawn3() {
        let handle = thread::spawn(|| {
            for i in 1..6 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
        if let Ok(_) = handle.join() {
            println!("OK");
        }
    }

    #[test]
    fn spawn4() {
        let vec = vec![1, 2, 3, 4, 5, 6];
        let handle = thread::spawn(move || {
            for i in &vec {
                println!("{}", i);
            }
            println!("{}", vec.len());
        });

        handle.join().unwrap();

        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }

    pub mod builder {
        use std::ops::Sub;
        use std::thread;
        use std::time::{Duration, Instant};

        #[test]
        fn new() {
            use std::thread;
            let builder = thread::Builder::new()
                .name("foo".into())
                .stack_size(32 * 1024);
            let handler = builder
                .spawn(|| {
                    println!("hello thread");
                })
                .unwrap();
            handler.join().unwrap();
            println!("over!");
        }

        #[test]
        fn name() {
            let builder = thread::Builder::new().name("foo".into());
            let start = Instant::now();
            let handler = builder
                .spawn(move || {
                    assert_eq!(thread::current().name(), Some("foo"));
                    println!("{}", thread::current().name().unwrap());
                    thread::sleep(Duration::from_secs(1))
                })
                .unwrap();

            let handler2 = thread::Builder::new()
                .spawn(|| {
                    println!("{}", "hello");
                    thread::sleep(Duration::from_secs(2));
                })
                .unwrap();

            let handler3 = thread::spawn(|| {
                println!("{}", "hello world");
                thread::sleep(Duration::from_secs(1));
            });
            handler.join().unwrap();
            handler2.join().unwrap();
            handler3.join().unwrap();
            let end = Instant::now();
            let x = end.sub(start);
            println!("{}", x.as_secs())
        }
    }
}
