pub mod string {
    use std::borrow::Cow;

    #[test]
    fn new() {
        let mut string = String::new();
        println!("{}", string);
        string.push('a');
        println!("{}", string)
    }

    #[test]
    fn with_capacity() {
        let mut string = String::with_capacity(10);
        println!("len {}", string.len()); //0
        println!("capacity {}", string.capacity()); //10

        for i in 0..string.capacity() {
            string.push((i as u8 + 65) as char);
        }
        println!("{}", string);
        println!("len {}", string.len());
        string.push('a');
        println!("len {}", string.len()); //11
        println!("len {}", string.capacity()); //20
    }

    #[test]
    fn from_utf8() {
        let result = String::from_utf8(vec![240, 159, 146, 150]);
        let string = match result {
            Ok(string) => {
                println!("match => {}", string);
                string
            }
            Err(_) => {
                println!("err");
                "".to_string()
            }
        };
        println!("{}", string)
    }

    #[test]
    fn from_utf8_lossy() {
        let sparkle_heart = vec![240, 159, 146, 150];
        let string = String::from_utf8_lossy(&sparkle_heart);
        println!("{}", string);
        match string {
            Cow::Borrowed(string) => println!("Borrowed => {}", string),
            Cow::Owned(string) => println!("Owned => {}", string),
        }

        let input = b"Hello \xF0\x90\x80World";
        let output = String::from_utf8_lossy(input);
        println!("{}", output)
    }

    #[test]
    fn into_bytes() {
        let vec = String::from("abcd").into_bytes();
        println!("{:?}", vec);
        println!("capacity : {}", vec.capacity());
        println!("len : {}", vec.len());
        println!("first : {:?}", vec.first().unwrap());
    }

    #[test]
    fn as_str() {
        let s = String::from("foo");
        println!("{}", s);
        println!("{}", s.as_str());
        println!("{}", s.as_str().len());
    }

    #[test]
    fn as_mut_str() {
        let mut string = String::from("hello rust");
        let x = string.as_mut_str();
        x.make_ascii_uppercase();
        println!("{}", string)
    }

    #[test]
    fn push_str() {
        let mut string = String::from("hello");
        string.push_str(" world");
        println!("{}", string)
    }

    #[test]
    fn reserve() {
        let mut string = String::from("hello world");
        println!("capacity {}", string.capacity()); //11
        println!("len {}", string.len()); //11
        string.reserve(8);
        println!("{}", string);
        println!("capacity {}", string.capacity()); //22
        println!("len {}", string.len()); //11
        string.reserve(15);
        println!("capacity {}", string.capacity()); //44
        println!("len {}", string.len()); //11
    }

    #[test]
    fn reserve_exact() {
        let mut string = String::from("hello world");
        println!("capacity {}", string.capacity()); //11
        println!("len {}", string.len()); //11
        string.reserve_exact(8);
        println!("{}", string);
        println!("capacity {}", string.capacity()); //19
        println!("len {}", string.len()); //11
        string.reserve_exact(15);
        println!("capacity {}", string.capacity()); //16
        println!("len {}", string.len()); //11
    }

    #[test]
    fn shrink_to_fit() {
        let mut string = String::with_capacity(40);
        println!("capacity {}", string.capacity()); // 40
        println!("len {}", string.len()); //0
        string.push_str("hello world");
        println!("capacity {}", string.capacity()); //40
        println!("len {}", string.len()); //11
        string.shrink_to_fit();
        println!("capacity {}", string.capacity()); //11
        println!("len {}", string.len()); //11
    }

    #[test]
    fn truncate() {
        let mut string = String::from("hello world");
        println!("{}", string); //hello world
        println!("len {}", string.len()); //11
        string.truncate(5);
        println!("{}", string); //hello
        println!("len {}", string.len()); //5
    }

    #[test]
    fn pop() {
        let mut string = String::from("hello world");
        println!("{}", string);
        println!("{:?}", string.pop());
        println!("{}", string.pop().unwrap());
        println!("{}", string);
        while let Some(i) = string.pop() {
            if i != ' ' {
                print!("{}", i)
            }
        }
    }

    #[test]
    fn remove() {
        let mut string = String::from("hello world");
        while string.is_empty() == false {
            println!("{}", string);
            string.remove(string.len() - 1);
        }
    }

    #[test]
    fn retain() {
        let mut s = String::from("abcde");
        let keep = [false, true, true, false, true];
        let mut i = 0;
        s.retain(|_| (keep[i], i += 1).0);
        println!("{}", s);
        let mut str = "rust golang".to_owned();
        str.retain(|c| c != ' ');
        println!("{}", str);
    }
}
