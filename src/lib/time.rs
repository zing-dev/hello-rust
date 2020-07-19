pub mod time {
    pub mod duration {
        use std::thread::sleep;
        use std::time::{Duration, Instant};

        #[test]
        fn constant() {}

        #[test]
        fn new() {
            let duration = Duration::new(1, 2);
            println!("{:?}", duration)
        }

        #[test]
        fn from_secs() {
            let duration = Duration::from_secs(1);
            println!("{:?}", duration);
        }

        #[test]
        fn from_millis() {
            let duration = Duration::from_millis(1000);
            println!("{:?}", duration);
        }

        #[test]
        fn from_micros() {
            let duration = Duration::from_micros(1000000);
            println!("{:?}", duration);
        }

        #[test]
        fn from_nanos() {
            let duration = Duration::from_nanos(1000000000);
            println!("{:?}", duration);
        }

        #[test]
        fn as_secs() {
            let instant = Instant::now();
            sleep(Duration::from_secs(1));
            let duration = instant.elapsed();
            println!("{:?}", duration);
            println!("{:?}", duration.as_secs())
        }

        #[test]
        fn subsec_millis() {
            let duration = Duration::from_millis(5432);
            println!("{:?}", duration.as_secs()); //5
            println!("{:?}", duration.subsec_millis()); //432
        }

        #[test]
        fn subsec_micros() {
            let duration = Duration::from_micros(1_234_567);
            println!("{:?}", duration.as_secs()); //1
            println!("{:?}", duration.subsec_micros()); //234567
        }

        #[test]
        fn subsec_nanos() {
            let duration = Duration::from_nanos(5_201_005_010);
            println!("{:?}", duration.as_secs()); //5
            println!("{:?}", duration.as_millis()); //5201
            println!("{:?}", duration.subsec_millis()); //201
            println!("{:?}", duration.subsec_micros()); //201005
            println!("{:?}", duration.subsec_nanos()); //201005010
        }

        #[test]
        fn checked_add() {
            let duration = Duration::new(1, 1);
            println!(
                "{:?}",
                duration.checked_add(Duration::from_secs(2)).unwrap()
            )
        }

        #[test]
        fn checked_sub() {
            let duration = Duration::new(10, 1);
            println!(
                "{:?}",
                duration.checked_sub(Duration::from_secs(5)).unwrap()
            )
        }

        #[test]
        fn checked_mul() {
            let duration = Duration::new(10, 1);
            println!("{:?}", duration.checked_mul(3)); //Some(30.000000003s)
            println!("{:?}", Duration::new(std::u64::MAX, 0).checked_mul(2));
        }

        #[test]
        fn checked_div() {
            println!("{:?}", Duration::new(2, 0).checked_div(2));
            println!("{:?}", Duration::new(1, 0).checked_div(2));
            println!("{:?}", Duration::new(2, 0).checked_div(0));
        }
    }

    pub mod instant {
        use std::thread::sleep;
        use std::time::{Duration, Instant};

        #[test]
        fn nwo() {
            let now = Instant::now();
            println!("{:#?}", now);
            println!("{:?}", now);
        }

        #[test]
        fn duration_since() {
            let now = Instant::now();
            sleep(Duration::from_secs(1));
            let new_now = Instant::now();
            println!("{:?}", new_now.duration_since(now));

            // panicked at 'supplied instant is later than self', src\libstd\time.rs:263:9
            //println!("{:?}", now.duration_since(new_now));
        }
    }
}
