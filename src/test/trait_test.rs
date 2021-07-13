pub mod trait_test {
    mod quick_start {
        struct Duck;

        struct Pig;

        trait Fly {
            fn fly(&self) -> bool;
        }

        impl Fly for Pig {
            fn fly(&self) -> bool {
                false
            }
        }

        impl Fly for Duck {
            fn fly(&self) -> bool {
                true
            }
        }

        // 静态分发
        // Rust 编译器会为fly_static::<Pig>(pig)和fly_static::<Duck>(duck)这两个具体类型的调用生成特殊化的代码
        fn fly_static<T: Fly>(s: T) -> bool {
            s.fly()
        }

        // 动态分发
        // 它会在运行时查找相应类型的方法,会带来一定的运行时开销,不过这种开销很小
        fn fly_dyn(s: &dyn Fly) -> bool {
            s.fly()
        }

        #[test]
        fn test() {
            let pig = Pig;
            assert_eq!(fly_static::<Pig>(pig), false);
            let duck = Duck;
            assert_eq!(fly_static::<Duck>(duck), true);
            assert_eq!(fly_dyn(&Pig), false);
            assert_eq!(fly_dyn(&Duck), true);
        }
    }

    mod common {
        struct Sheep {
            naked: bool,
            name: &'static str,
        }

        trait Animal {
            // Static method signature; `Self` refers to the implementor type.
            fn new(name: &'static str) -> Self;

            // Instance method signatures; these will return a string.
            fn name(&self) -> &'static str;
            fn noise(&self) -> &'static str;

            // Traits can provide default method definitions.
            fn talk(&self) {
                println!("{} says {}", self.name(), self.noise());
            }
        }

        impl Sheep {
            fn is_naked(&self) -> bool {
                self.naked
            }

            fn shear(&mut self) {
                if self.is_naked() {
                    // Implementor methods can use the implementor's trait methods.
                    println!("{} is already naked...", self.name());
                } else {
                    println!("{} gets a haircut!", self.name);

                    self.naked = true;
                }
            }
        }

        // Implement the `Animal` trait for `Sheep`.
        impl Animal for Sheep {
            // `Self` is the implementor type: `Sheep`.
            fn new(name: &'static str) -> Sheep {
                Sheep { name, naked: false }
            }

            fn name(&self) -> &'static str {
                self.name
            }

            fn noise(&self) -> &'static str {
                if self.is_naked() {
                    "baaaaah?"
                } else {
                    "baaaaah!"
                }
            }

            // Default trait methods can be overridden.
            fn talk(&self) {
                // For example, we can add some quiet contemplation.
                println!("{} pauses briefly... {}", self.name, self.noise());
            }
        }

        fn new(a: &mut Sheep) {
            a.talk();
            a.shear();
            a.talk();
        }

        fn animal(a: &impl Animal) {
            println!("{}", a.name())
        }

        #[test]
        fn run() {
            // Type annotation is necessary in this case.
            let mut dolly: Sheep = Animal::new("Dolly");
            dolly.talk();
            dolly.shear();
            dolly.talk();

            new(&mut dolly);
            animal(&dolly);
            animal(&dolly);
        }
    }

    mod derive {
        use std::hash::Hash;

        // `Centimeters`, a tuple struct that can be compared
        #[derive(PartialEq, PartialOrd)]
        struct Centimeters(f64);

        impl Centimeters {
            fn to_inches(&self) -> Inches {
                let Centimeters(f) = *self;
                Inches((f / 2.54) as i32)
            }
        }

        // `Inches`, a tuple struct that can be printed
        #[derive(Debug, PartialEq, PartialOrd)]
        struct Inches(i32);

        impl Inches {
            fn to_centimeters(&self) -> Centimeters {
                let &Inches(inches) = self;

                Centimeters(inches as f64 * 2.54)
            }
        }

        // `Seconds`, a tuple struct with no additional attributes
        struct Seconds(i32);

        #[test]
        fn run() {
            let _one_second = Seconds(1);

            // Error: `Seconds` can't be printed; it doesn't implement the `Debug` trait
            //println!("One second looks like: {:?}", _one_second);
            // TODO ^ Try uncommenting this line

            // Error: `Seconds` can't be compared; it doesn't implement the `PartialEq` trait
            //let _this_is_true = (_one_second == _one_second);
            // TODO ^ Try uncommenting this line

            let foot = Inches(12);

            println!("One foot equals {:?}", foot);

            let meter = Centimeters(100.0);
            let cmp = if foot.to_centimeters() < meter {
                "smaller"
            } else {
                "bigger"
            };
            println!("One foot is {} than one meter.", cmp);

            let cmp = if meter.to_inches() < foot {
                "smaller"
            } else {
                "bigger"
            };
            println!("One meter is {} than one foot.", cmp);
        }
    }

    mod dyn_test {
        use rand::Rng;

        struct Sheep {}

        struct Cow {}

        trait Animal {
            // Instance method signature
            fn noise(&self) -> &'static str;
        }

        // Implement the `Animal` trait for `Sheep`.
        impl Animal for Sheep {
            fn noise(&self) -> &'static str {
                "baaaaah!"
            }
        }

        // Implement the `Animal` trait for `Cow`.
        impl Animal for Cow {
            fn noise(&self) -> &'static str {
                "moooooo!"
            }
        }

        // Returns some struct that implements Animal, but we don't know which one at compile time.
        fn random_animal(random_number: f64) -> Box<dyn Animal> {
            if random_number < 0.5 {
                Box::new(Sheep {})
            } else {
                Box::new(Cow {})
            }
        }

        #[test]
        fn run() {
            let random_number = rand::thread_rng().gen_range(0.0..1.0);
            let animal = random_animal(random_number);
            println!(
                "You've randomly chosen an animal, and it says {}",
                animal.noise()
            );
        }
    }

    mod ops {
        use std::ops;
        use std::ops::Add;

        struct Foo;

        struct Bar;

        #[derive(Debug)]
        struct FooBar;

        #[derive(Debug)]
        struct BarFoo;

        #[derive(Debug)]
        struct BarFooFooBar;

        // The `std::ops::Add` trait is used to specify the functionality of `+`.
        // Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
        // The following block implements the operation: Foo + Bar = FooBar
        impl ops::Add<FooBar> for BarFoo {
            type Output = BarFooFooBar;

            fn add(self, _rhs: FooBar) -> BarFooFooBar {
                BarFooFooBar
            }
        }

        impl ops::Add<Bar> for Foo {
            type Output = FooBar;

            fn add(self, _rhs: Bar) -> FooBar {
                println!("> Foo.add(Bar) was called");

                FooBar
            }
        }

        // By reversing the types, we end up implementing non-commutative addition.
        // Here, we make `Add<Foo>` - the trait for addition with a RHS of type `Foo`.
        // This block implements the operation: Bar + Foo = BarFoo
        impl ops::Add<Foo> for Bar {
            type Output = BarFoo;

            fn add(self, _rhs: Foo) -> BarFoo {
                println!("> Bar.add(Foo) was called");

                BarFoo
            }
        }

        #[derive(Debug, PartialEq)]
        struct Point<T> {
            x: T,
            y: T,
        }

        // Notice that the implementation uses the associated type `Output`.
        impl<T: Add<Output = T>> Add for Point<T> {
            //关联类型Output 必须指定具体类型
            type Output = Self;
            fn add(self, other: Self) -> Self::Output {
                Self {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }

        // Rust遵循一条重要的规则：孤儿规则(Orphan Rule)
        // 如果要实现某个trait,该trait和要实现该trait的类型至少有一个要在当前crate中定义
        trait MyAdd<RHS = Self> {
            type Output;
            fn my_add(self, rhs: RHS) -> Self::Output;
        }

        impl MyAdd<u64> for u32 {
            type Output = u64;
            fn my_add(self, rhs: u64) -> Self::Output {
                self as u64 + rhs
            }
        }

        // trait继承
        trait Page {
            fn set_page(&mut self, p: i32) {
                println!("page default:{}", p)
            }
        }

        trait PerPage {
            fn set_per_page(&mut self, p: i32) {
                println!("per page default:{}", p)
            }
        }

        trait Paginate: Page + PerPage {
            fn set_skip_page(&mut self, num: i32) {
                println!("skip page: {}", num)
            }
        }

        struct MyPage {
            page: i32,
        }

        impl Page for MyPage {}

        impl PerPage for MyPage {}

        //impl Paginate for MyPage {}

        impl<T: Page + PerPage> Paginate for T {}

        #[test]
        fn run() {
            println!("Foo + Bar = {:?}", Foo + Bar);
            println!("Bar + Foo = {:?}", Bar + Foo);
            println!("Bar + Foo + Foo + Bar = {:?}", (Bar + Foo) + (Foo + Bar));
            println!("{:?}", Point { x: 1, y: 2 } + Point { x: 11, y: 12 });
            println!("{:?}", 1.my_add(2));

            let mut p = MyPage { page: 10 };
            p.set_page(1);
            p.set_per_page(2);
            p.set_skip_page(20)
        }
    }

    mod traits {
        pub trait Summary {
            fn summarize(&self) -> String;
        }

        pub struct Article {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }

        impl Summary for Article {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }
        }

        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }

        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        fn from_summary(s: &impl Summary) {
            println!("{}", s.summarize());
        }

        fn from_summary2(s: &dyn Summary) {
            println!("{}", s.summarize());
        }

        fn to_summary() -> impl Summary {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course, as you probably already know, people"),
                reply: false,
                retweet: false,
            }
        }

        fn to_summary2() -> Box<dyn Summary> {
            Box::new(to_summary())
        }

        #[test]
        fn test() {
            let tweet = Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course, as you probably already know, people"),
                reply: false,
                retweet: false,
            };
            from_summary(&tweet);
            from_summary2(&tweet);

            println!("{}", to_summary().summarize());
            println!("{}", to_summary2().summarize());
        }
    }
}
