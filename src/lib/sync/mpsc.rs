pub mod mpsc {
    use std::sync::mpsc;
    use std::sync::mpsc::channel;
    use std::thread;
    use std::time::Duration;

    pub fn hello_world() {
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || {
            let mut i = 0;
            loop {
                if i > 20 { break; }
                sender.send(i).unwrap();
                println!("send: {}", i);
                i += 1;
                thread::sleep(Duration::from_secs(1));
            }
        });

        loop {
            match receiver.recv() {
                Ok(v) => println!("read: {}", v),
                Err(e) => {
                    println!("err: {}", e.to_string());
                    return;
                }
            };
        }
    }

    #[test]
    fn test_hello_world() {
        hello_world()
    }

    #[allow(unused_must_use)]
    pub fn hello_world2() {
        let (sender, receiver) = mpsc::channel::<i32>();
        let job = thread::spawn(move || {
            loop {
                println!("start received: ");
                match receiver.recv() {
                    Ok(v) => println!("read: {}", v),
                    Err(e) => {

                        println!("err: {}", e.to_string());
                        return;
                    }
                };
            }
        });

        let mut i = 0;
        loop {
            if i > 10 { break; }
            sender.send(i).unwrap();
            println!("send: {}", i);
            i += 1;
            thread::sleep(Duration::from_secs(1));
        }
        println!("----> start over");
        drop(sender);
        job.join();
    }

    #[test]
    fn test_hello_world2() {
        hello_world2()
    }
}