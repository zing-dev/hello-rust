#[test]
pub fn hello_world() {
    println!("Hello World");
    println!("Hello Rust");
}

#[test]
fn format() {
    let s1 = format!("test");
    let s2 = format!("hello {}", "world!");
    let s3 = format!("x = {}, y = {y}", 10, y = 30);
    println!("{},{},{}", s1, s2, s3)
}

#[test]
fn print() {
    use std::io::{self, Write};
    print!("same as format! but the text is printed to the console (io::stdout).");
    print!("\n");
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
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix. The number 31i64 for example has the type i64.

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object = "the lazy dog",
             subject = "the quick brown fox",
             verb = "jumps over");

    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number = 1, width = 6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number = 1, width = 6);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    // println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ Add the missing argument: "James"
    println!("My name is {0}, {1} {0}", "Bond", "Zing");

    // Create a structure named `Structure` which contains an `i32`.
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.
    println!("This struct `{:#?}` print...", Structure(3));
}

#[test]
fn debug() {
    use std::fmt::{self, Formatter, Result};

    struct Language {
        name: String,
        age: u8,
    }

    impl fmt::Debug for Language {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "debug=>{},{}", self.name, self.age)
        }
    }
    impl fmt::Display for Language {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "display=> {},{}", self.name, self.age)
        }
    }

    let rust = Language { name: "Language: rust".to_owned(), age: 8 };
    println!("{}", rust);
    println!("{:?}", rust);
    println!("{:#?}", rust);

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    // Pretty print
    println!("{:#?}", peter);
}

#[test]
fn display() {
    use std::fmt; // Import `fmt`

    // A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
    #[derive(Debug)]
    struct MinMax(i64, i64);

    // Implement `Display` for `MinMax`.
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Use `self.number` to refer to each positional data point.
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    // Define a structure where the fields are nameable for comparison.
    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    // Similarly, implement `Display` for `Point2D`
    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Customize so only `x` and `y` are denoted.
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    impl fmt::Binary for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Customize so only `x` and `y` are denoted.
            write!(f, "x: {:b}, y: {:b}", self.x as i64, self.y as i64)
        }
    }

    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 13.3, y: 17.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    println!("Debug: {:b}", point);
}

#[test]
fn list() {
    use std::fmt::{self, Debug, Display};
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Extract the value using tuple indexing,
            // and create a reference to `vec`.
            let vec = &self.0;

            write!(f, "[")?;

            // Iterate over `v` in `vec` while enumerating the iteration
            // count in `count`.
            for (count, v) in vec.iter().enumerate() {
                // For every element except the first, add a comma.
                // Use the ? operator, or try!, to return on errors.
                if count != 0 { write!(f, ", ")?; }
                write!(f, "{:?}", v)?;
            }

            // Close the opened bracket and return a fmt::Result value.
            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
