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
    }
}
