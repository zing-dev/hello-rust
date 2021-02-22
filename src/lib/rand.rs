pub mod rand {
    use rand::{Rng, RngCore};

    #[test]
    fn next_u32() {
        let mut arr = [0i8; 20];
        rand::thread_rng().fill(&mut arr);
        println!("{:?}", arr);
        println!("{}", rand::thread_rng().next_u32());
        println!("{}", rand::thread_rng().next_u64());
    }

    #[test]
    fn thread_rng() {
        let rng = rand::thread_rng();
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
        println!("{}", rng.gen_range(1..10));
        println!("{}", rng.gen_range(1.0..10.1))
    }

    #[test]
    fn random() {
        println!("i8 {}", rand::random::<i8>());
        println!("u8 {}", rand::random::<u8>());
        println!("i16 {}", rand::random::<i16>());
    }
}
