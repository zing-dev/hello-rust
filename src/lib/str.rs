pub mod str_test {
    use std::str;

    // • str， 表示固定长度的字符串。
    // • String， 表示可增长的字符串。
    // • CStr， 表示由C分配而被Rust借用的字符串，一般用于和C语言交互。
    // • CString， 表示由 Rust 分配且可以传递给 C 函数使用的 C 字符串，同样用于和 C 语言交互。
    // • OsStr， 表示和操作系统相关的字符串。这是为了兼容 Windows系统。
    // • OsString，表示 OsStr 的可变版本。与 Rust 字符串可以相互转换。
    // • Path，表示路径，定义于 std::path模块中。Path包装了 OsStr。
    // • PathBuf. 跟 Path 配对，是 Path 的可变版本。PathBuf包装了 OsString。

    // ·静态存储区。有代表性的是字符串字面量，&'static str类型的字符串被直接存储到己编译的可执行文件中，随着程序一起加载启动。
    // ·堆分配。 如果&str类型的字符串是通过堆String类型的字符串取切片生成的，则存储在堆上。
    //  因为 String 类型的字符串是堆分配的，&str只不过是其在堆上的切片。
    // ·栈分配。 比如使用 str::from_utf8方法，就可以将战分配的[u8;N]数组转换为一个&str 字符串

    // String类型 由三部分组成:指向堆中字节序列的指针 Cas_p位方法)、 记录 堆中字节序列的字节长度 C!en方法) 和堆分配的容量 (capacity方法〉
    #[test]
    fn test_string() {
        let mut str = String::from("hello world");
        println!(
            "{:p},{:p},{} {}",
            str.as_ptr(),
            &str,
            str.len(),
            str.capacity()
        );

        str.reserve(10);
        println!(
            "{:p},{:p},{} {}",
            str.as_ptr(),
            &str,
            str.len(),
            str.capacity()
        );
        //as_ptr 获取的是堆中字节序列的指针地址
        //引用 &a 的地址为字符串变量在栈上指针的地址，
        //len方法获取的是堆中字节序列的字节数，非字符个数

        let string: String = String::new();
        assert_eq!("", string);
        let string: String = String::from("hello rust");
        assert_eq!("hello rust", string);
        let string: String = String::with_capacity(20);
        assert_eq!("", string);
        let str: &'static str = "the tao of rust";
        let string: String = str.chars().filter(|c| !c.is_whitespace()).collect();
        assert_eq!("thetaoofrust", string);
        let string: String = str.to_owned();
        assert_eq!("the tao of rust", string);
        let string: String = str.to_string();
        let str: &str = &string[11..15];
        assert_eq!("rust", str);
    }

    #[test]
    fn handle_string() {
        let str = "hello";
        let mut chars = str.chars();
        assert_eq!(Some('h'), chars.next());
        assert_eq!(Some('e'), chars.next());
        assert_eq!(Some('l'), chars.next());
        assert_eq!(Some('l'), chars.next());
        assert_eq!(Some('o'), chars.next());
        let mut bytes = str.bytes();
        assert_eq!(5, str.len());
        for byte in bytes {
            println!("{}", byte)
        }
    }

    #[test]
    fn test() {
        let hello_world = "Hello, World!";
        println!("{}", hello_world);
        let hello_world: &'static str = "Hello, world!";
        println!("{}", hello_world);
        let tao = str::from_utf8(&[0xE9u8, 0x81u8, 0x93u8]).unwrap();
        println!("{}", tao); //道
        println!("{}", String::from("\u{9053}"))
    }

    #[test]
    fn into_boxed_bytes() {
        let s = "this is a string";
        let boxed_str = s.to_owned().into_boxed_str();
        let boxed_bytes = boxed_str.into_boxed_bytes();
        assert_eq!(*boxed_bytes, *s.as_bytes());
    }

    #[test]
    fn replace() {
        let s = "this is old";
        assert_eq!("this is new", s.replace("old", "new"));

        let s = "this is old";
        assert_eq!(s, s.replace("cookie monster", "little lamb"));
    }

    #[test]
    fn replacen() {
        let s = "foo foo 123 foo";
        assert_eq!("new new 123 foo", s.replacen("foo", "new", 2));
        assert_eq!("faa fao 123 foo", s.replacen('o', "a", 3));
        assert_eq!("foo foo new23 foo", s.replacen(char::is_numeric, "new", 1));
    }

    #[test]
    fn to_lowercase() {
        let s = "HELLO";
        assert_eq!("hello", s.to_lowercase());

        let sigma = "Σ";
        assert_eq!("σ", sigma.to_lowercase());

        // but at the end of a word, it's ς, not σ:
        let odysseus = "ὈΔΥΣΣΕΎΣ";
        assert_eq!("ὀδυσσεύς", odysseus.to_lowercase());

        let new_year = "农历新年";
        assert_eq!(new_year, new_year.to_lowercase());
    }

    #[test]
    fn into_string() {
        let string = String::from("birthday gift");
        let boxed_str = string.clone().into_boxed_str();
        assert_eq!(boxed_str.into_string(), string);
    }

    #[test]
    fn repeat() {
        println!("{}", "hello ".repeat(2))
    }

    mod from_str {

        #[test]
        fn test() {
            use std::str::FromStr;

            assert_eq!(FromStr::from_str("true"), Ok(true));
            assert_eq!(FromStr::from_str("false"), Ok(false));
            assert!(<bool as FromStr>::from_str("not even a boolean").is_err());
            assert!(<bool as FromStr>::from_str("true").is_ok());
            println!("{}", <bool as FromStr>::from_str("true").unwrap());
            println!("{}", bool::from_str("true").unwrap());

            assert_eq!("true".parse(), Ok(true));
            assert_eq!("false".parse(), Ok(false));
            assert!("not even a boolean".parse::<bool>().is_err());
        }

        #[test]
        fn from_str() {
            use std::num::ParseIntError;
            use std::str::FromStr;

            #[derive(Debug, PartialEq)]
            struct Point {
                x: i32,
                y: i32,
            }

            impl FromStr for Point {
                type Err = ParseIntError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    let coords: Vec<&str> = s
                        .trim_matches(|p| p == '(' || p == ')')
                        .split(',')
                        .collect();

                    let x = coords[0].parse::<i32>()?;
                    let y = coords[1].parse::<i32>()?;

                    Ok(Point { x, y })
                }
            }

            let p = Point::from_str("(1,2)").unwrap();
            assert_eq!(p, Point { x: 1, y: 2 });
            println!("{:?}", p);

            let s = "5";
            let x = i32::from_str(s).unwrap();
            println!("{}", x);
        }
    }

    #[test]
    fn valid_up_to() {
        use std::str;

        // some invalid bytes, in a vector
        let sparkle_heart = vec![0, 159, 146, 150];

        // std::str::from_utf8 returns a Utf8Error
        let error = str::from_utf8(&sparkle_heart).unwrap_err();

        // the second byte is invalid here
        assert_eq!(1, error.valid_up_to());
    }

    #[test]
    fn from_utf8() {
        use std::str;

        // some bytes, in a vector
        let sparkle_heart = vec![240, 159, 146, 150];

        // We know these bytes are valid, so just use `unwrap()`.
        let sparkle_heart = str::from_utf8(&sparkle_heart).unwrap();

        assert_eq!("💖", sparkle_heart);
    }

    #[test]
    fn from_utf8_mut() {
        use std::str;

        // "Hello, Rust!" as a mutable vector
        let mut hellorust = vec![72, 101, 108, 108, 111, 44, 32, 82, 117, 115, 116, 33];

        // As we know these bytes are valid, we can use `unwrap()`
        let outstr = str::from_utf8_mut(&mut hellorust).unwrap();

        assert_eq!("Hello, Rust!", outstr);

        // Some invalid bytes in a mutable vector
        let mut invalid = vec![128, 223];

        assert!(str::from_utf8_mut(&mut invalid).is_err());
    }
}
