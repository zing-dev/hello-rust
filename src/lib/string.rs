pub mod string {
    use std::borrow::Cow;
    use std::ops::Add;
    use std::string::FromUtf8Error;

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
}
