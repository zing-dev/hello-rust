pub mod result {
    #[test]
    pub fn test() {
        #[derive(Debug)]
        enum Version {
            Version1,
            Version2,
        }

        fn parse_version(header: &[u8]) -> Result<Version, &'static str> {
            match header.get(0) {
                None => Err("invalid header length"),
                Some(&1) => Ok(Version::Version1),
                Some(&2) => Ok(Version::Version2),
                Some(_) => Err("invalid version"),
            }
        }

        let version = parse_version(&[1, 2, 3, 4]);
        match version {
            Ok(v) => println!("working with version: {:?}", v),
            Err(e) => println!("error parsing header: {:?}", e),
        }
    }

    #[test]
    fn test2() {
        let good_result: Result<i32, i32> = Ok(10);
        let bad_result: Result<i32, i32> = Err(10);

        // The `is_ok` and `is_err` methods do what they say.
        assert!(good_result.is_ok() && !good_result.is_err());
        assert!(bad_result.is_err() && !bad_result.is_ok());

        // `map` consumes the `Result` and produces another.
        let good_result: Result<i32, i32> = good_result.map(|i| i + 1);
        let bad_result: Result<i32, i32> = bad_result.map(|i| i - 1);

        // Use `and_then` to continue the computation.
        let good_result: Result<bool, i32> = good_result.and_then(|i| Ok(i == 11));

        // Use `or_else` to handle the error.
        let bad_result: Result<i32, i32> = bad_result.or_else(|i| Ok(i + 20));
        println!("{:?}", bad_result);
        // Consume the result and return the contents with `unwrap`.
        let final_awesome_result = good_result.unwrap();
        println!("{}", final_awesome_result);
    }

    #[test]
    fn test3() {
        use std::fs::File;
        use std::io;
        use std::io::prelude::*;

        struct Info {
            name: String,
            age: i32,
            rating: i32,
        }

        #[allow(unused_must_use)]
        fn write_info(info: &Info) -> io::Result<()> {
            // Early return on error
            let mut file = match File::create("my_best_friends.txt") {
                Err(e) => return Err(e),
                Ok(f) => f,
            };
            if let Err(e) = file.write_all(format!("name: {}\n", info.name).as_bytes()) {
                return Err(e);
            }
            if let Err(e) = file.write_all(format!("age: {}\n", info.age).as_bytes()) {
                return Err(e);
            }
            if let Err(e) = file.write_all(format!("rating: {}\n", info.rating).as_bytes()) {
                return Err(e);
            }
            Ok(())
        }

        let result = write_info(&Info {
            name: String::from("zing"),
            age: 26,
            rating: 100,
        });
        println!("{:?}", result);
        // note: `#[warn(unused_must_use)]` on by default
        // note: this `Result` may be an `Err` variant, which should be handled

        fn write_info2(info: &Info) -> io::Result<()> {
            let mut file = File::create("my_best_friends.txt")?;
            // Early return on error
            file.write_all(format!("name: {}\n", info.name).as_bytes())?;
            file.write_all(format!("age: {}\n", info.age).as_bytes())?;
            file.write_all(format!("rating: {}\n", info.rating).as_bytes())?;
            Ok(())
        }

        let result = write_info(&Info {
            name: String::from("hello world"),
            age: 26,
            rating: 100,
        });
        println!("{:?}", result);
    }

    #[test]
    fn is_ok() {
        let x: Result<i32, &str> = Ok(-3);
        assert_eq!(x.is_ok(), true);

        let x: Result<i32, &str> = Err("Some error message");
        assert_eq!(x.is_ok(), false);
    }

    #[test]
    fn is_err() {
        let x: Result<i32, &str> = Ok(-3);
        assert_eq!(x.is_err(), false);

        let x: Result<i32, &str> = Err("Some error message");
        assert_eq!(x.is_err(), true);
    }

    #[test]
    fn ok() {
        let x: Result<u32, &str> = Ok(2);
        assert_eq!(x.ok(), Some(2));

        let x: Result<u32, &str> = Err("Nothing here");
        assert_eq!(x.ok(), None);
    }

    #[test]
    fn err() {
        let x: Result<u32, &str> = Ok(2);
        assert_eq!(x.err(), None);

        let x: Result<u32, &str> = Err("Nothing here");
        assert_eq!(x.err(), Some("Nothing here"));
    }

    #[test]
    fn as_ref() {
        let x: Result<u32, &str> = Ok(2);
        assert_eq!(x.as_ref(), Ok(&2));

        let x: Result<u32, &str> = Err("Error");
        assert_eq!(x.as_ref(), Err(&"Error"));
    }

    #[test]
    fn as_mut() {
        fn mutate(r: &mut Result<i32, i32>) {
            match r.as_mut() {
                Ok(v) => *v = 42,
                Err(e) => *e = 0,
            }
        }

        let mut x: Result<i32, i32> = Ok(2);
        mutate(&mut x);
        assert_eq!(x.unwrap(), 42);

        let mut x: Result<i32, i32> = Err(13);
        mutate(&mut x);
        assert_eq!(x.unwrap_err(), 0);
    }

    #[test]
    fn map() {
        let line = "1\n2\n3\n4\n";

        for num in line.lines() {
            match num.parse::<i32>().map(|i| i * 2) {
                Ok(n) => println!("{}", n),
                Err(..) => {}
            }
        }
    }

    #[test]
    fn map_or() {
        let x: Result<_, &str> = Ok("foo");
        assert_eq!(x.map_or(42, |v| v.len()), 3);

        let x: Result<&str, _> = Err("bar");
        assert_eq!(x.map_or(42, |v| v.len()), 42);
    }

    #[test]
    fn map_or_else() {
        let k = 21;

        let x: Result<_, &str> = Ok("foo");
        assert_eq!(x.map_or_else(|_| k * 2, |v| v.len()), 3);

        let x: Result<&str, _> = Err("bar");
        assert_eq!(x.map_or_else(|&_| k * 2, |v| v.len()), 42);
    }

    #[test]
    fn map_err() {
        fn stringify(x: u32) -> String {
            format!("error code: {}", x)
        }

        let x: Result<u32, u32> = Ok(2);
        assert_eq!(x.map_err(stringify), Ok(2));

        let x: Result<u32, u32> = Err(13);
        assert_eq!(x.map_err(stringify), Err("error code: 13".to_string()));
    }

    #[test]
    fn iter() {
        let x: Result<u32, &str> = Ok(7);
        assert_eq!(x.iter().next(), Some(&7));

        let x: Result<u32, &str> = Err("nothing!");
        assert_eq!(x.iter().next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut x: Result<u32, &str> = Ok(7);
        match x.iter_mut().next() {
            Some(v) => *v = 40,
            None => {}
        }
        assert_eq!(x, Ok(40));

        let mut x: Result<u32, &str> = Err("nothing!");
        assert_eq!(x.iter_mut().next(), None);
    }

    #[test]
    fn and() {
        let x: Result<u32, &str> = Ok(2);
        let y: Result<&str, &str> = Err("late error");
        assert_eq!(x.and(y), Err("late error"));

        let x: Result<u32, &str> = Err("early error");
        let y: Result<&str, &str> = Ok("foo");
        assert_eq!(x.and(y), Err("early error"));

        let x: Result<u32, &str> = Err("not a 2");
        let y: Result<&str, &str> = Err("late error");
        assert_eq!(x.and(y), Err("not a 2"));

        let x: Result<u32, &str> = Ok(2);
        let y: Result<&str, &str> = Ok("different result type");
        assert_eq!(x.and(y), Ok("different result type"));
    }

    #[test]
    fn and_then() {
        fn sq(x: u32) -> Result<u32, u32> {
            Ok(x * x)
        }
        fn err(x: u32) -> Result<u32, u32> {
            Err(x)
        }

        assert_eq!(Ok(2).and_then(sq).and_then(sq), Ok(16));
        assert_eq!(Ok(2).and_then(sq).and_then(err), Err(4));
        assert_eq!(Ok(2).and_then(err).and_then(sq), Err(2));
        assert_eq!(Err(3).and_then(sq).and_then(sq), Err(3));
    }

    #[test]
    fn or() {
        let x: Result<u32, &str> = Ok(2);
        let y: Result<u32, &str> = Err("late error");
        assert_eq!(x.or(y), Ok(2));

        let x: Result<u32, &str> = Err("early error");
        let y: Result<u32, &str> = Ok(2);
        assert_eq!(x.or(y), Ok(2));

        let x: Result<u32, &str> = Err("not a 2");
        let y: Result<u32, &str> = Err("late error");
        assert_eq!(x.or(y), Err("late error"));

        let x: Result<u32, &str> = Ok(2);
        let y: Result<u32, &str> = Ok(100);
        assert_eq!(x.or(y), Ok(2));
    }

    #[test]
    fn or_else() {
        fn sq(x: u32) -> Result<u32, u32> {
            Ok(x * x)
        }
        fn err(x: u32) -> Result<u32, u32> {
            Err(x)
        }

        assert_eq!(Ok(2).or_else(sq).or_else(sq), Ok(2));
        assert_eq!(Ok(2).or_else(err).or_else(sq), Ok(2));
        assert_eq!(Err(3).or_else(sq).or_else(err), Ok(9));
        assert_eq!(Err(3).or_else(err).or_else(err), Err(3));
    }

    #[test]
    fn unwrap_or() {
        let default = 2;
        let x: Result<u32, &str> = Ok(9);
        assert_eq!(x.unwrap_or(default), 9);

        let x: Result<u32, &str> = Err("error");
        assert_eq!(x.unwrap_or(default), default);
    }

    #[test]
    fn unwrap_or_else() {
        fn count(x: &str) -> usize {
            x.len()
        }

        assert_eq!(Ok(2).unwrap_or_else(count), 2);
        assert_eq!(Err("foo").unwrap_or_else(count), 3);
    }
}
