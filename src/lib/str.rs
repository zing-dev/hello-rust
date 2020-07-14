pub mod str {
    #[test]
    fn test() {
        let hello_world = "Hello, World!";
        println!("{}", hello_world);
        let hello_world: &'static str = "Hello, world!";
        println!("{}", hello_world);
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
