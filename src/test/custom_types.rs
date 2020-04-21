use crate::test::custom_types::List::*;
#[test]
fn struct_test() {
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    struct Unit;

    struct Pair(i32, f32);

    struct Point {
        x: f32,
        y: f32,
    }

    #[allow(dead_code)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    let name = "Zing";
    let age = 26;
    let zing = Person { name, age };
    println!("{:#?}", zing);

    let point = Point { x: 100.1, y: 101.1 };
    println!("point => {},{}", point.x, point.y);

    let bottom_right = Point { x: 1.1, ..point };
    println!("bottom_right => {},{}", bottom_right.x, bottom_right.y);

    let Point {
        x: top_edge,
        y: left_edge,
    }: Point = point;
    println!("{},{}", top_edge, left_edge);

    let _unit = Unit;
    // println!("{:#?}", _unit);
    let pair = Pair(1, 1.1);
    println!("{},{}", pair.0, pair.1);
    let Pair(integer, decimal) = pair;
    println!("integer:{},decimal:{}", integer, decimal)
}

#[test]
fn enum_test() {
    enum WebEvent {
        PageLoad,
        PageUnLoad,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 },
    }

    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnLoad => println!("page unload"),
            WebEvent::KeyPress(c) => println!("pressed {}", c),
            WebEvent::Paste(str) => println!("pasted {}", str),
            WebEvent::Click { x, y } => println!("clicked at x {}, y{}", x, y),
        }
    }

    let pressed = WebEvent::KeyPress('😄');
    let pasted = WebEvent::Paste("Hello World".to_owned());
    let click = WebEvent::Click { x: 11, y: 20 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnLoad;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

#[test]
fn alias() {
    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }

    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

    impl Operations {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }
    let x = Operations::Add;
    println!("{}", x.run(1, 2));

    let y = VeryVerboseEnumOfThingsToDoWithNumbers::Subtract;
    println!("{}", y.run(11, 2));
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

enum Language {
    Rust,
    Golang,
    Java,
    C,
    Cpp,
}

#[test]
fn use_test() {
    fn run() {
        // use crate::test::custom_types::Language;
        // use crate::test::custom_types::Status::{Poor, Rich};
        // use crate::test::custom_types::Work::*;
        use crate::test::custom_types::{
            Language,
            Status::*,
            Work::{Civilian, Soldier},
        };

        let status = Poor;
        let work = Civilian;

        match status {
            Rich => println!("the rich have lots of money!"),
            Poor => println!("the poor have no money!"),
        }

        match work {
            Civilian => println!("civilian work!"),
            Work => println!("soldiers fight"),
        }

        let learn = Language::Rust;
        match learn {
            Language::Rust => println!("rust"),
            Language::Golang => println!("golang"),
            Language::Java => println!("java"),
            Language::C => println!("c"),
            Language::Cpp => println!("cpp"),
        }
    }
    run()
}

#[test]
fn c_like() {
    enum Number {
        Zero,
        One,
        Two,
    }
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000FF,
    }

    fn run() {
        println!("zero {}", Number::Zero as i32);
        println!("one {}", Number::One as i32);
        println!("roses are #{:06x}", Color::Red as i32);
        println!("violets are #{:06x}", Color::Blue as i32);
    }
    run();
}

#[derive(Debug)]
enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    /*fn append(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }*/

    /*fn remove(&mut self) -> List {
        match &self {
            Cons(elem, tail) => {
                self = ***tail;
                tail
            }
            Nil => self,
        }
    }*/

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
            Nil => format!("Nil"),
        }
    }
}

#[test]
fn linked_list() {
    fn run() {
        // Create an empty linked list
        let mut list = List::new();

        // Prepend some elements
        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);

        // Show the final state of the list
        println!("linked list has length: {}", list.len());
        println!("{}", list.stringify());
        println!("{:#?}", list);
        // list.remove();
        // println!("{}", list.stringify());
    }
    run();
}

#[test]
fn constants() {
    static LANGUAGE: &str = "Rust";
    const THRESHOLD: i32 = 10;

    fn is_big(n: i32) -> bool {
        // Access constant in some function
        n > THRESHOLD
    }
    let n = 16;
    println!("{}", LANGUAGE);
    println!("{}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
}
