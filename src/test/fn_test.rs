pub mod fn_test {
    pub mod closures {
        use rand::Rng;

        #[test]
        fn common() {
            fn function(i: i32) -> i32 {
                i + 1
            }
            let closure_annotated = |i: i32| -> i32 { i + 1 };
            let closure_inferred = |i| i + 1;

            let i = 1;
            println!("function: {}", function(i));
            println!("closure_annotated: {}", closure_annotated(i));
            println!("closure_inferred: {}", closure_inferred(i));

            let one = || 1;
            println!("closure returning one: {}", one());
            println!("{}", { || 1 }());
            println!("{}", { || "string" }());
            println!("{}", { || "string".to_uppercase() }());
        }

        #[test]
        fn capture() {
            let name = "zing";
            let print = || println!("name is {}", name);
            print();

            let _name2 = name;
            print();

            let _name3 = &name;
            print();

            println!("name is {}", name);

            let mut count = 0;
            let mut inc = || {
                count += 1;
                println!("count {}", count);
            };
            inc();

            //^^^^^^ immutable borrow occurs here
            //let _reborrow = &count;
            inc();
            let _reborrow = &mut count;
            //inc();
        }

        // A function which takes a closure as an argument and calls it.
        // <F> denotes that F is a "Generic type parameter"
        fn apply<F>(f: F)
        where
            // The closure takes no input and returns nothing.
            F: FnOnce(),
            // F: Fn(),
        {
            // ^ TODO: Try changing this to `Fn` or `FnMut`.
            f();
        }

        // A function which takes a closure and returns an `i32`.
        fn apply_to_3<F>(f: F) -> i32
        where
            // The closure takes an `i32` and returns an `i32`.
            F: Fn(i32) -> i32,
        {
            f(3)
        }

        #[test]
        fn input_parameters() {
            use std::mem;
            let greeting = "hello";
            // A non-copy type.
            // `to_owned` creates owned data from borrowed one
            let mut farewell = "goodbye".to_owned();

            // Capture 2 variables: `greeting` by reference and
            // `farewell` by value.
            let diary = || {
                // `greeting` is by reference: requires `Fn`.
                println!("I said {}.", greeting);

                // Mutation forces `farewell` to be captured by
                // mutable reference. Now requires `FnMut`.
                farewell.push_str("!!!");
                println!("Then I screamed {}.", farewell);
                println!("Now I can sleep. zzzzz");

                // Manually calling drop forces `farewell` to
                // be captured by value. Now requires `FnOnce`.
                mem::drop(farewell);
            };

            // Call the function which applies the closure.
            apply(diary);

            // `double` satisfies `apply_to_3`'s trait bound
            let double = |x| 2 * x;

            println!("3 doubled: {}", apply_to_3(double));
        }

        mod anonymity {
            // `F` must implement `Fn` for a closure which takes no
            // inputs and returns nothing - exactly what is required
            // for `print`.
            fn apply<F>(f: F)
            where
                F: Fn(),
            {
                f();
            }
            fn apply2<F>(f: F)
            where
                F: FnOnce(),
            {
                f();
            }

            fn apply3<F>(mut f: F)
            where
                F: FnMut() -> i8,
            {
                println!("{}", f());
            }

            #[test]
            fn main() {
                let x = 7;

                // Capture `x` into an anonymous type and implement
                // `Fn` for it. Store it in `print`.
                let print = || println!("{}", x);

                apply(print);
                apply2(print);

                let a = 10;
                let f = || a + 2;
                apply3(f);
            }
        }

        mod input_functions {
            // Define a function which takes a generic `F` argument
            // bounded by `Fn`, and calls it
            fn call_me<F: Fn()>(f: F) {
                f();
            }

            // Define a wrapper function satisfying the `Fn` bound
            fn function() {
                println!("I'm a function!");
            }

            #[test]
            fn main() {
                // Define a closure satisfying the `Fn` bound
                let closure = || println!("I'm a closure!");

                call_me(closure);
                call_me(function);
            }
        }

        mod output_parameters {
            fn create_fn() -> impl Fn() {
                let text = "Fn".to_owned();

                move || println!("This is a: {}", text)
            }

            fn create_fnmut() -> impl FnMut() {
                let text = "FnMut".to_owned();

                move || println!("This is a: {}", text)
            }

            fn create_fnonce() -> impl FnOnce() {
                let text = "FnOnce".to_owned();

                move || println!("This is a: {}", text)
            }

            #[test]
            fn main() {
                let fn_plain = create_fn();
                let mut fn_mut = create_fnmut();
                let fn_once = create_fnonce();

                fn_plain();
                fn_mut();
                fn_once();
            }

            #[test]
            fn run() {
                let v = vec!["563913.060", "563913.080"];
                let x: Vec<f32> = v.iter().map(|f| f.parse::<f32>().unwrap()).collect();
                println!("{:?}", x);
                println!("{}", "563913.060".parse::<f32>().unwrap());
                println!("{}", "563913.090".parse::<f32>().unwrap());
                println!("{}", "563913.050".parse::<f32>().unwrap());
                println!("{}", "563913.100".parse::<f32>().unwrap());
                println!("{}", "563913.060".parse::<f64>().unwrap());
                println!("{}", "563913.090".parse::<f64>().unwrap());
                println!("{}", "1.0001".parse::<f32>().unwrap());
                println!("{}", "123456789.0001".parse::<f32>().unwrap());
            }
        }
    }

    mod fn_once {
        use serde::export::fmt::{Debug, Display};

        fn consume_with_relish<F: FnOnce() -> String>(func: F) {
            // `func` consumes its captured variables, so it cannot be run more
            // than once.
            println!("Consumed: {}", func());
            println!("Delicious!");
            // Attempting to invoke `func()` again will throw a `use of moved
            // value` error for `func`.
            //func();
        }

        fn func<A, F>(f: F)
        where
            A: Display + Debug,
            F: FnOnce() -> A,
        {
            println!("{:?}", f());
            //println!("{:?}", f());
        }

        #[test]
        fn test() {
            let x = String::from("x");
            let consume_and_return_x = move || x;
            consume_with_relish(consume_and_return_x);

            let i = 10;
            let f = || i * i;
            func(f);

            let t = (10, 20);
            let f2 = || t.0 * t.1;
            func(f2);

            let t2 = ("hello", " ", "word");
            let f3 = move || t2.0.to_owned() + t2.1 + t2.2;
            func(f3);

            let x = String::from("x");
            let f = move || x;
            func(f);
        }
    }

    mod fn_mut {
        use std::ops::Add;

        fn do_twice<F>(mut func: F)
        where
            F: FnMut(),
        {
            func();
            func();
            func();
            func();
        }

        #[test]
        fn test() {
            let mut x: usize = 1;
            {
                let add_two_to_x = || x += 2;
                do_twice(add_two_to_x);
            }
            // assert_eq!(x, 5);

            x = 72;
            let mut string = String::new();
            let str = || {
                // string.insert_str(x, "hello ");
                //string.add("hello ");
                string.push(x as u8 as char);
                x += 1;
            };
            do_twice(str);
            //do_twice(str);
            println!("{}", string);
        }
    }
    mod fn_test {
        fn call_with_one<F>(func: F) -> usize
        where
            F: Fn(usize) -> usize,
        {
            func(1)
        }

        #[test]
        fn test() {
            let double = |x| x * 2;
            assert_eq!(call_with_one(double), 2);
        }
    }
}
