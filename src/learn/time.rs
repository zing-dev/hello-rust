#[cfg(test)]
mod time {
    use std::time::*;
    use std::thread::sleep;
    use std::ops::Add;

    #[test]
    fn instant_monotonic() {
        let a = Instant::now();
        let b = Instant::now();
        assert!(b >= a);
    }

    #[test]
    fn second() {
        let five_seconds = Duration::new(5, 0);
        // both declarations are equivalent
        assert_eq!(Duration::new(5, 0), Duration::from_secs(5));
        println!("{:?}", five_seconds);//5s
        println!("{:?}", five_seconds.as_secs());//5
        println!("{:?}", five_seconds.as_micros());
    }

    #[test]
    fn instant(){
        let now = Instant::now();
        println!("=================");
        // we sleep for 2 seconds
        sleep(Duration::new(2, 0));
        // it prints '2'
        println!("{}", now.elapsed().as_secs());
    }

    #[test]
    fn system_time(){
        let now = SystemTime::now();
        // we sleep for 2 seconds
        sleep(Duration::new(2, 0));
        match now.elapsed() {
            Ok(elapsed) => {
                // it prints '2'
                println!("{}", elapsed.as_secs());
            }
            Err(e) => {
                // an error occurred!
                println!("Error: {:?}", e);
            }
        }
    }

    #[test]
    fn system_time_error(){
        let sys_time = SystemTime::now();
        sleep(Duration::from_secs(1));
        let new_sys_time = SystemTime::now();
        match sys_time.duration_since(new_sys_time) {
            Ok(_) => {}
            Err(e) => println!("SystemTimeError difference: {:?}", e.duration()),
        }
    }

    #[test]
    fn duration_since(){
        let now = Instant::now();
        sleep(Duration::new(1, 0));
        let new_now = Instant::now();
        println!("{:?}", new_now.duration_since(now));
    }

    #[test]
    fn elapsed(){
        let sys_time = SystemTime::now();
        let one_sec = Duration::from_secs(1);
        sleep(one_sec);
        assert!(sys_time.elapsed().unwrap() >= one_sec);
        sleep(one_sec * 2);
        assert!(sys_time.elapsed().unwrap() >= one_sec * 2);
    }

    #[test]
    fn checked_add(){
        let sys_time = SystemTime::now();
        let option = sys_time.checked_add(Duration::from_secs(1));
        println!("{:?}",option.unwrap())
    }

    #[test]
    fn checked_sub(){
        let sys_time = SystemTime::now();
        let option = sys_time.checked_sub(Duration::from_secs(1));
        println!("{:?}",option.unwrap())
    }

    #[test]
    fn add(){
        let sys_time = SystemTime::now();
        let time = sys_time.add(Duration::from_secs(1));
        println!("{:?}",time)
    }
}