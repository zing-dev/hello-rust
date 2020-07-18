pub mod os_string {
    use std::ffi::{OsStr, OsString};

    /**
       OsString
       OsStr  Borrowed reference to an OS string
    */

    #[test]
    fn new() {
        let string = OsString::new();
        println!("{:?}", string);
    }

    #[test]
    fn from() {
        let string = OsString::from("hello world");
        println!("{:?}", string);
        println!("{:?}", string.len());

        println!("{:?}", OsString::from("a").len()); //1
        println!("{:?}", OsString::from("ς").len()); //2
        println!("{:?}", OsString::from("啊").len()); //3
        println!("{:?}", OsString::from("☎").len()); //3
        println!("{:?}", OsString::from("😘").len()); //4
    }

    #[test]
    fn as_os_str() {
        let string = OsString::from("hello world");
        println!("{:?}", string);
        println!("{:?}", string.as_os_str());
        println!("{:?}", string.as_os_str().len());

        assert_eq!(string, string.as_os_str());
        println!("{:?}", OsString::from("🌞").as_os_str().len());
    }

    #[test]
    fn into_string() {
        let string = OsString::from("rust");
        if let Ok(string) = string.into_string() {
            println!("{}", string)
        }
    }

    #[test]
    fn push() {
        let mut string = OsString::default();
        string.push("hello ");
        string.push("world ");
        println!("{}", string.into_string().unwrap())
    }

    #[test]
    fn with_capacity() {
        let mut os_string = OsString::with_capacity(10);
        let capacity = os_string.capacity();
        // This push is done without reallocating
        os_string.push("foo");
        assert_eq!(capacity, os_string.capacity());
    }

    #[test]
    fn clear() {
        let mut string = OsString::from("👌");
        println!("len {}", string.len()); //4
        println!("capacity {}", string.capacity()); //4
        string.clear();
        println!("len {}", string.len()); //0
        println!("capacity {}", string.capacity()); //4
    }

    #[test]
    fn reserve() {
        let mut s = OsString::new();
        s.reserve(10);
        assert!(s.capacity() >= 10);
        s.reserve(5);
        println!("{}", s.capacity())
    }

    #[test]
    fn reserve_exact() {
        let mut s = OsString::new();
        s.reserve_exact(10);
        assert!(s.capacity() >= 10);
        s.reserve_exact(5);
        println!("{}", s.capacity()); //10
        s.reserve_exact(15);
        println!("{}", s.capacity()); //15
    }

    #[test]
    fn shrink_to_fit() {
        let mut s = OsString::from("foo");

        s.reserve(100);
        assert!(s.capacity() >= 100);

        s.shrink_to_fit();
        assert_eq!(3, s.capacity());
        s.push("₯");
        s.shrink_to_fit();
        println!("{}", s.capacity())
    }

    #[test]
    fn into_boxed_os_str() {
        let s = OsString::from("hello");
        let b: Box<OsStr> = s.into_boxed_os_str();
        println!("{:?}", b);
    }
}
