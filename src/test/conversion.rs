pub mod conversion {
    use std::convert::{TryFrom, TryInto};
    use std::fmt::{Debug, Display, Formatter};
    use std::io;
    use std::num;
    use std::{fmt, fs};

    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(value: i32) -> Self {
            Number { value }
        }
    }

    #[derive(Debug)]
    struct Text {
        value: String,
    }

    impl From<String> for Text {
        fn from(value: String) -> Self {
            Text { value }
        }
    }

    enum CliError {
        IoError(io::Error),
        ParseError(num::ParseIntError),
    }

    impl From<io::Error> for CliError {
        fn from(error: io::Error) -> Self {
            CliError::IoError(error)
        }
    }

    impl From<num::ParseIntError> for CliError {
        fn from(error: num::ParseIntError) -> Self {
            CliError::ParseError(error)
        }
    }

    fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
        let contents = fs::read_to_string(&file_name)?;
        println!("{}", contents);
        let num: i32 = contents.trim().parse()?;
        Ok(num)
    }

    #[test]
    fn from() {
        let number = Number::from(12);
        println!("{:#?}", number);

        let text = Text::from("123".to_owned());
        println!("{:#?}", text);

        let result = open_and_parse_file("Cargo.toml");
        match result {
            Ok(num) => println!("read {}", num),
            Err(err) => match err {
                CliError::IoError(err) => println!(" io => {}", err.to_string()),
                CliError::ParseError(err) => println!(" parse => {}", err.to_string()),
            },
        }
    }

    #[test]
    fn into() {
        let i = 10;
        let n: Number = i.into();
        println!("{:#?}", n);
        let t: Text = "hello".to_owned().into();
        println!("{:#?}", t)
    }

    //////////////////////////////////////////////////////////////////////////
    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    #[test]
    fn try_test() {
        assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
        assert_eq!(EvenNumber::try_from(5), Err(()));
        // TryInto
        let result: Result<EvenNumber, ()> = 8i32.try_into();
        assert_eq!(result, Ok(EvenNumber(8)));
        match result {
            Ok(EvenNumber(x)) => println!("event: {}", x),
            Err(()) => {}
        }
        let result: Result<EvenNumber, ()> = 5i32.try_into();
        assert_eq!(result, Err(()));
    }

    struct Circle {
        radius: i32,
    }

    impl Display for Circle {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }

    #[test]
    fn string() {
        let circle = Circle { radius: 6 };
        println!("{}", circle.to_string());

        let parsed: i32 = "5".parse::<i64>().unwrap() as i32;
        let turbo_parsed = "10".parse::<i32>().unwrap();

        let sum = parsed + turbo_parsed;
        println!("Sum: {:?}", sum);
    }
}
