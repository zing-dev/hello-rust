#[cfg(test)]
#[allow(unused_variables)]
#[allow(unused_must_use)]
pub mod macros {
    use core::fmt::Write as CoreFmtWrite;
    use std::fmt;
    use std::fmt::Write as FmtWrite;
    use std::io::{self, Result as IOResult, Write};

    #[allow(unreachable_code)]
    #[test]
    fn panic() {
        panic!();
        panic!("this is a terrible mistake!");
        panic!(4); // panic with the value of 4 to be collected elsewhere
        panic!("this is a {} {message}", "fancy", message = "message");
    }

    #[test]
    fn print() {
        print!("this ");
        print!("will ");
        print!("be ");
        print!("on ");
        print!("the ");
        print!("same ");
        print!("line ");
        io::stdout().flush().unwrap();
        print!("this string has a newline, why not choose println! instead?\n");
        io::stdout().flush().unwrap();
    }

    #[test]
    fn println() {
        println!(); // prints just a newline
        println!("hello there!");
        println!("format {} arguments", "some");
    }

    #[test]
    fn eprint() {
        eprint!("Error: Could not complete task");
        eprint!("");
    }

    #[test]
    fn eprintln() {
        eprintln!("Error: Could not complete task");
    }

    #[test]
    fn assert_eq() {
        let a = 2;
        let b = dbg!(a * 2) + 1;
        //      ^-- prints: [src/main.rs:2] a * 2 = 4
        assert_eq!(b, 5);
        assert_eq!(5, 5);
        assert_eq!(true, !false);
        assert_eq!("", "");
    }

    #[test]
    fn dbg() {
        fn foo(n: usize) {
            if let Some(_) = dbg!(n.checked_sub(4)) {
                // ...
            }
        }
        foo(3);

        fn factorial(n: u32) -> u32 {
            if dbg!(n <= 1) {
                dbg!(1)
            } else {
                dbg!(n * factorial(n - 1))
            }
        }
        dbg!(factorial(4));

        assert_eq!(dbg!(1usize, 2u32), (1, 2));

        assert_eq!(1, dbg!(1u32,)); // trailing comma ignored
        assert_eq!((1,), dbg!((1u32,))); // 1-tuple
    }

    #[test]
    fn assert_ne() {
        let a = 3;
        let b = 2;
        assert_ne!(a, b);
        assert_ne!(a, b, "we are testing that the values are not equal");
    }

    #[test]
    fn debug_assert() {
        // the panic message for these assertions is the stringified value of the
        // expression given.
        debug_assert!(true);
        fn some_expensive_computation() -> bool {
            true
        } // a very simple function
        debug_assert!(some_expensive_computation());
        // assert with a custom message
        let x = true;
        debug_assert!(x, "x wasn't true!");
        let a = 3;
        let b = 27;
        debug_assert!(a + b == 30, "a = {}, b = {}", a, b);
    }

    #[test]
    fn debug_assert_eq() {
        let a = 3;
        let b = 1 + 2;
        debug_assert_eq!(a, b);
    }

    #[test]
    fn write() -> io::Result<()> {
        fn write() -> std::io::Result<()> {
            let mut w = Vec::new();
            write!(&mut w, "test")?;
            write!(&mut w, "formatted {}", "arguments")?;
            assert_eq!(w, b"testformatted arguments");
            Ok(())
        }
        write()?;

        let mut s = String::new();
        let mut v = Vec::new();
        write!(&mut s, "{} {}", "abc", 123); // uses fmt::Write::write_fmt
        write!(&mut v, "s = {:?}", s); // uses io::Write::write_fmt
        assert_eq!(v, b"s = \"abc 123\"");

        //todo
        #[derive(Debug)]
        struct Example;
        impl CoreFmtWrite for Example {
            fn write_str(&mut self, _s: &str) -> core::fmt::Result {
                write!(_s.to_string(), "{:?} {}", self, _s)
            }
        }
        let mut m = Example {};
        write!(&mut m, "Hello World").expect("Not written");
        println!("{:?}", m);
        Ok(())
    }

    #[test]
    fn writeln() {
        fn main() -> IOResult<()> {
            let mut w = Vec::new();
            writeln!(&mut w)?;
            writeln!(&mut w, "test")?;
            writeln!(&mut w, "formatted {}", "arguments")?;
            assert_eq!(&w[..], "\ntest\nformatted arguments\n".as_bytes());
            Ok(())
        }
        main();

        fn main2() -> Result<(), Box<dyn std::error::Error>> {
            let mut s = String::new();
            let mut v = Vec::new();
            writeln!(&mut s, "{} {}", "abc", 123)?; // uses fmt::Write::write_fmt
            writeln!(&mut v, "s = {:?}", s)?; // uses io::Write::write_fmt
            assert_eq!(v, b"s = \"abc 123\\n\"\n");
            Ok(())
        }
        main2();
    }

    #[test]
    fn unreachable() {
        #[allow(dead_code)]
        fn foo(x: Option<i32>) {
            match x {
                Some(n) if n >= 0 => println!("Some(Non-negative)"),
                Some(n) if n < 0 => println!("Some(Negative)"),
                Some(_) => unreachable!(), // compile error if commented out
                None => println!("None"),
            }
        }

        fn divide_by_three(x: u32) -> u32 {
            // one of the poorest implementations of x/3
            for i in 0.. {
                if 3 * i < i {
                    panic!("u32 overflow");
                }
                if x < 3 * i {
                    return i - 1;
                }
            }
            unreachable!();
        }
    }

    #[test]
    fn format_args() {
        let s = fmt::format(format_args!("hello {}", "world"));
        let s2 = format_args!("hello {}", "world");
        println!("{}", s);
        println!("{}", format_args!("hello {}", "world"));
        assert_eq!(s, format!("hello {}", "world"));
    }

    #[test]
    fn env() {
        let path: &str = env!("PATH");
        println!("the $PATH variable at the time of compiling was: {}", path);
    }

    #[test]
    fn option_env() {
        let key: Option<&'static str> = option_env!("SECRET_KEY");
        println!("the secret key might be: {:?}", key);
        let key: Option<&'static str> = option_env!("GOPATH");
        println!("the secret key might be: {:?}", key);
        println!("the secret key might be: {:?}", key.unwrap());
    }

    #[test]
    fn concat() {
        let s = concat!("test", 10, 'b', true);
        assert_eq!(s, "test10btrue");

        println!(concat!(123, true, false))
    }

    #[test]
    fn current_line() {
        let current_line = line!();
        println!("defined on line: {}", current_line);
    }

    #[test]
    fn column() {
        let current_column = column!();
        println!("defined on column: {}", current_column);
    }

    #[test]
    fn file() {
        let this_file = file!();
        println!("defined in file: {}", this_file);
    }

    #[test]
    fn stringify() {
        let one_plus_one = stringify!(1 + 1);
        assert_eq!(one_plus_one, "1 + 1");
        println!(stringify!(true));
        println!(stringify!(1.1.1.1));
    }

    #[test]
    fn assert() {
        assert!(true);
        fn some_computation() -> bool {
            true
        } // a very simple function
        assert!(some_computation());
        // assert with a custom message
        let x = true;
        assert!(x, "x wasn't true!");
        let a = 3;
        let b = 27;
        assert!(a + b >= 30, "a = {}, b = {}", a, b);
    }
}
