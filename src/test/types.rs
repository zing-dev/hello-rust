#[test]
fn cast() {
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    //let integer: u8 = decimal;

    // Explicit conversion
    let integer = decimal as u8;

    //only `u8` can be cast as `char`, not `f32`
    //let character = integer as char;

    // Error! There are limitations in conversion rules. A float cannot be directly converted to a char.
    // let character = decimal as u8 as char;
    let character = decimal as u8 as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type, T,
    // T::MAX + 1 is added or subtracted until the value
    // fits into the new type

    // 1000 already fits in a u16
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated.
    //println!("1000 as a u8 is : {}", 1000 as u8);
    #[allow(overflowing_literals)]
    let a = 1000 as u8;
    println!("1000 as a u8 is : {}", a);

    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative.

    // Unless it already fits, of course.
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 as u8 -> 128, whose two's complement in eight bits is:
    //println!(" 128 as a i8 is : {}", 128 as i8);

    // repeating the example above
    // 1000 as u8 -> 232
    //println!("1000 as a u8 is : {}", 1000 as u8);
    // and the two's complement of 232 is -24
    //println!(" 232 as a i8 is : {}", 232 as i8);
}

#[test]
fn literals() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literal, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    struct A {}
    struct B();
    struct C(i8, i8);

    let a = A {};
    let b = B();
    let c = C(1, 2);
    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&a));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&b));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&c));
}

#[test]
fn inference() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2_i32);
    println!("{:#?}", vec);

    println!("{:#?}", vec![1, 2, 3, 4, 5]);
    println!("{:#?}", vec!["rust", "go", "java"]);
}

#[test]
fn alias() {
    // `NanoSecond` is a new name for `u64`.
    type NanoSecond = u64;
    type Inch = u64;

    // Use an attribute to silence warning.
    #[allow(non_camel_case_types)]
    type u64_t = u64;
    // `NanoSecond` = `Inch` = `u64_t` = `u64`.
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
