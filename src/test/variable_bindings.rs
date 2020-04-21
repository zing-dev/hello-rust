#[test]
fn mutability() {
    let mut a = 10;
    let b = 11;
    println!("a {}", a);
    println!("b {}", b);
    a = 11;
    println!("a {}", a);
    //b = 12; //cannot assign twice to immutable variable
    //println!("a {}", b);
}

#[test]
fn scope() {
    let long_lived_binding = 1;
    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }
    // help: a local variable with a similar name exists: `long_lived_binding`
    // not found in this scope
    //println!("outer short: {}", short_lived_binding);
    println!("outer long: {}", long_lived_binding);
    let long_lived_binding = 'a';
    println!("outer long: {}", long_lived_binding);
}

#[test]
fn declare() {
    // Declare a variable binding
    let a_binding;
    {
        let x = 2;
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Error! Use of uninitialized binding
    //println!("another binding: {}", another_binding);
    another_binding = 1;
    println!("another binding: {}", another_binding);
}

#[test]
fn freeze() {
    let mut _mutable_integer = 7_i32;
    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        //_mutable_integer = 50;

        // `_mutable_integer` goes out of scope
    }

    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
}
