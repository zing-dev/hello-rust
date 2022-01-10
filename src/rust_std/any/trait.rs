#[allow(unused_imports)]
use std::any::{Any, TypeId};

#[test]
fn type_id_test() {
    fn is_string(s: &dyn Any) -> bool {
        TypeId::of::<String>() == s.type_id()
    }

    assert_eq!(is_string(&0), false);
    assert_eq!(is_string(&"cookie monster".to_string()), true);
}

#[test]
fn is_test() {
    fn is_string(t: &dyn Any) {
        if t.is::<String>() {
            println!("{:?} is string", t)
        } else {
            println!("{:?} is not string", t)
        }
    }

    is_string(&String::from("hi"));
    is_string(&"hello".to_string());
    is_string(&"hello");
    is_string(&true);
    is_string(&'a')
}

#[test]
//如果装箱值的类型为T，则返回对该值的引用；如果不是，则返回None
fn downcast_ref() {
    fn print_if_string(s: &dyn Any) {
        if let Some(string) = s.downcast_ref::<String>() {
            println!("It's a string({}): '{}'", string.len(), string);
        } else {
            println!("Not a string...{:?}", s);
        }
    }

    print_if_string(&0);
    print_if_string(&"cookie monster".to_string());
}

#[test]
// 如果装箱值的类型为T，则返回该值的一些可变引用；如果不是，则返回None。
fn downcast_mut() {
    fn modify_if_u32(s: &mut dyn Any) {
        if let Some(num) = s.downcast_mut::<u32>() {
            *num = 42;
        }
    }

    let mut x = 10u32;
    let mut s = "hello world".to_string();

    modify_if_u32(&mut x);
    modify_if_u32(&mut s);
    assert_eq!(x, 42);
    assert_eq!(&s, "hello world");
}
