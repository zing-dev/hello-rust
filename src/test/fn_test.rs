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
        }
    }
}
