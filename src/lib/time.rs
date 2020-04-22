pub mod time {
    pub mod instant {
        use std::time::{Instant, Duration};
        use std::thread::sleep;

        #[test]
        fn nwo() {
            let now = Instant::now();
            println!("{:#?}", now);
            println!("{:?}", now);
        }

        #[test]
        fn duration_since(){
            let now = Instant::now();
            sleep(Duration::new(1, 0));
            let new_now = Instant::now();
            println!("{:?}", new_now.duration_since(now));
        }
    }
}
