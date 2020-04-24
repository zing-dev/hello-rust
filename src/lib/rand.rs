pub mod rand {
    use rand::Rng;

    #[warn(unused_doc_comments)]
    #[test]
    fn thread_rng() {
        let mut rng = rand::thread_rng();
        /**
        ThreadRng {
            rng: RefCell {
                value: ReseedingRng {
                    rng: StdRng {
                        rng: Isaac64Rng {},
                    },
                    generation_threshold: 32768,
                    bytes_generated: 0,
                    reseeder: ThreadRngReseeder,
                },
            },
        }
        */
        println!("{:#?}", rng)
    }

    #[test]
    fn gen() {
        let mut rng = rand::thread_rng();
        println!("i8 {}", rng.gen::<i8>());
        println!("i16 {}", rng.gen::<i16>());
        println!("i32 {}", rng.gen::<i32>());
        println!("i64 {}", rng.gen::<i64>());

        println!("u8 {}", rng.gen::<u8>());
    }

    #[test]
    fn gen_range() {
        let mut rng = rand::thread_rng();
        println!("{}", rng.gen_range(1, 10));
        println!("{}", rng.gen_range(1.0, 10.1))
    }

    #[test]
    fn gen_iter() {
        println!(
            "{:?}",
            rand::thread_rng()
                .gen_iter::<i8>()
                .take(10)
                .collect::<Vec<i8>>()
        );

        println!(
            "{:?}",
            rand::thread_rng()
                .gen_iter::<u8>()
                .take(5)
                .collect::<Vec<u8>>()
        );

        println!(
            "{:?}",
            rand::thread_rng()
                .gen_iter::<(u8, bool)>()
                .take(5)
                .collect::<Vec<(u8, bool)>>()
        )
    }

    #[test]
    fn gen_weighted_bool() {
        println!("{:?}", rand::thread_rng().gen_weighted_bool(10));
        println!("{:?}", rand::thread_rng().gen_weighted_bool(3));
        println!("{:?}", rand::thread_rng().gen_weighted_bool(0));
        println!("{:?}", rand::thread_rng().gen_weighted_bool(1));
    }

    #[test]
    fn gen_ascii_chars() {
        println!(
            "{:?}",
            rand::thread_rng()
                .gen_ascii_chars()
                .take(10)
                .collect::<String>()
        );
        println!(
            "{:?}",
            rand::thread_rng()
                .gen_ascii_chars()
                .take(10)
                .collect::<Vec<char>>()
        );
    }

    #[test]
    fn random() {
        println!("i8 {}", rand::random::<i8>());
        println!("u8 {}", rand::random::<u8>());
        println!("i16 {}", rand::random::<i16>());
    }

    #[test]
    fn fill_bytes() {
        let mut rng = rand::thread_rng();
        let mut v = [0u8; 10];
        rng.fill_bytes(&mut v);
        println!("{:?}", &v[..]);
    }
}
