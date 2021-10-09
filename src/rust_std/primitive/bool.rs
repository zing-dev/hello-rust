#[test]
fn basic_usage_test() {
    let b = true || false & true;
    println!("{}", b);

    let default: bool = Default::default();
    println!("{}", default);

    match b {
        true => println!("b is true"),
        false => println!("b is false")
    }

    println!("{}", true as i32);
    println!("{}", false as i32);
}

#[test]
fn then_test() {
    assert_eq!(false.then(|| 0), None);
    assert_eq!(true.then(|| 0), Some(0));
    println!("{}", (100 > 10).then(|| 100).unwrap());
}

#[test]
fn from_test() {
    println!("{}", bool::from(true));
    println!("{}", bool::from(false));
}


#[test]
fn into_test() {
    let b: bool = true.into();
    println!("{}", b);
    let b: i32 = true.into();
    println!("{}", b);
}