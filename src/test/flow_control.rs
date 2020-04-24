pub mod if_else {
    use rand::{random, Rng};

    #[test]
    fn common() {
        let n = rand::thread_rng().gen::<i8>();
        if n < 0 {
            print!("{} is negative", n);
        } else if n > 0 {
            print!("{} is positive", n);
        } else {
            print!("{} is zero", n);
        }
    }

    #[test]
    fn test() {
        let n = rand::thread_rng().gen::<i8>();
        let big_n = if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            10 * n
        } else {
            println!(", and is a big number, halve the number");
            n / 2
        };
        println!("{} -> {}", n, big_n);
    }
}

pub mod loop_test {
    use rand::Rng;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn common() {
        let mut i = 0;
        loop {
            let str: String = rand::thread_rng().gen_ascii_chars().take(5).collect();
            println!("str = > {}", str);
            if i == 3 {
                println!("continue");
                continue;
            }
            if i > 10 {
                println!("break");
                break;
            }
            i += 1;
        }
    }

    #[test]
    #[allow(unreachable_code)]
    fn nested() {
        'outer: loop {
            println!("Entered the outer loop");

            'inner: loop {
                println!("Entered the inner loop");

                // This would break only the inner loop
                //break;

                // This breaks the outer loop
                break 'outer;
            }

            println!("This point will never be reached");
        }

        println!("Exited the outer loop");
    }

    #[test]
    fn return_test() {
        let mut counter = 0;
        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };
        println!("{},{}", counter, result);

        println!(
            "gt 10  => {}",
            loop {
                let a = rand::thread_rng().gen::<u8>();
                if a > 10 {
                    break a;
                }
            }
        )
    }
}

#[test]
fn while_test() {
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("15  => {}", n);
        } else if n % 3 == 0 {
            println!("3 ==> {}", n);
        } else if n % 5 == 0 {
            println!("5 ==> {}", n);
        } else if n == 97 {
            println!("break {}", n);
            break;
        } else {
            println!("none ==> {}", n);
        }
        n += 1;
    }
    println!("{}", n)
}

pub mod for_test {
    #[test]
    fn range() {
        for n in 1..21 {
            if n % 15 == 0 {
                println!("15  => {}", n);
            } else if n % 3 == 0 {
                println!("3 ==> {}", n);
            } else if n % 5 == 0 {
                println!("5 ==> {}", n);
            } else {
                println!("{}", n);
            }
        }
        println!("==========={}=============", !!2);
        println!("==========={}=============", !1);
        println!("==========={}=============", !0);
        println!("==========={}=============", !-1);
        println!("==========={}=============", !-2);
        for n in !1..=5 {
            println!("{}", n);
        }
    }

    #[test]
    fn iterators() {
        let names = vec!["Bob", "Frank", "Ferris"];
        for name in names.iter() {
            match name {
                &"Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }

        println!("names: {:?}", names);

        for name in names.into_iter() {
            match name {
                "Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }
        //value borrowed here after move
        //println!("{:?}", names);

        let mut names = vec!["Bob", "Frank", "Ferris"];
        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "There is a rustacean among us!",
                _ => "Hello",
            }
        }
        println!("names: {:?}", names);
    }
}

pub mod match_test {
    use rand::Rng;

    #[test]
    fn common() {
        let number = rand::thread_rng().gen_range(0, 30);
        println!("Tell me about {}", number);
        match number {
            1 => println!("One!"),
            2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
            13..=19 => println!("A teen"),
            _ => println!("Ain't special"),
        }

        let boolean = true;
        let binary = match boolean {
            // The arms of a match must cover all the possible values
            false => 0,
            true => 1,
        };
        println!("{} -> {}", boolean, binary);
    }

    #[test]
    fn tuples() {
        let t: (bool, i8) = rand::thread_rng().gen();
        println!("{},{}", t.0, t.1);
        match t {
            (true, i) if i >= 0 => println!("true => {}", i),
            (true, i) if i < 0 => println!("true => {}", i),
            (false, i) => println!("false => {}", i),
            (b, i) => println!("{}, {}", b, i),
        }
    }

    #[test]
    fn binding() {
        fn age() -> u32 {
            rand::thread_rng().gen_range(0, 30)
        }
        println!("Tell me what type of person you are");
        match age() {
            0 => println!("I'm not born yet I guess"),
            // Could `match` 1 ..= 12 directly but then what age
            // would the child be? Instead, bind to `n` for the
            // sequence of 1 ..= 12. Now the age can be reported.
            n @ 1..=12 => println!("I'm a child of age {:?}", n),
            n @ 13..=19 => println!("I'm a teen of age {:?}", n),
            // Nothing bound. Return the result.
            n => println!("I'm an old person of age {:?}", n),
        }

        fn some_number() -> Option<u32> {
            Some(age() + 10)
        }

        match some_number() {
            // Got `Some` variant, match if its value, bound to `n`,
            // is equal to 42.
            Some(n @ 42) => println!("The Answer: {}!", n),
            // Match any other number.
            Some(n) => println!("Not interesting... {}", n),
            // Match anything else (`None` variant).
            _ => (),
        }
    }
}
