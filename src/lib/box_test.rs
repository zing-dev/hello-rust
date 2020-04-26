pub mod box_test {
    use std::cmp::Ordering;

    #[test]
    fn new() {
        let x = Box::new(10);
        println!("{:?}", x);
        println!("{}", x);

        println!("{}", Box::new("Hello World"))
    }

    #[test]
    fn default() {
        let x: Box<u8> = Box::default();
        println!("{}", x);
        let x = Box::default() as Box<bool>;
        println!("{}", x);
    }

    #[test]
    fn clone() {
        let str = Box::from(String::from("rust")) as Box<String>;
        let str2 = str.clone();
        println!("{}", str); //rust
        println!("{}", str == str2); //true
        println!("{}", str.as_ptr() == str2.as_ptr()); //false
    }

    #[test]
    fn from() {
        println!("{}", Box::new(10) == Box::from(10));
        println!("{}", Box::new(1).eq(&Box::from(1)));
        let x = Box::new(1).cmp(&Box::from(2));
        match x {
            Ordering::Less => println!("less"),
            Ordering::Equal => println!("equal"),
            Ordering::Greater => println!("greater"),
        }

        let mut x = Box::from([1_u8, 2, 3]);
        println!("{:?}", x);
        println!("{}", x.first().unwrap());
    }
}
