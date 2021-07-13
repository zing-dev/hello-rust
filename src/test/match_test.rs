#[test]
fn quick_start() {
    let number = 42;
    match number {
        0 => println!("0"),
        1...3 => println!("1...3"),
        5 | 7 | 13 => println!("5|7|13"),
        n @ 42 => println!("n:{}", n),
        _ => println!("default"),
    }
}

#[test]
fn tuples() {
    let a = (1, 2);
    match a {
        (1, y) => println!("y {}", y),
        (0, y) => println!("y {}", y),
        (x, 1) => println!("x {}", x),
        (x, y) => println!("{},{}", x, y),
    }
}

#[test]
fn enums() {
    enum Language {
        Rust(String, i64),
        C(String, i8),
        CPP,
        Golang,
        Java(),
        PHP(String),
    }
    let r = Language::Rust("Rust".to_owned(), 10);
    match r {
        Language::Rust(s, i) => println!("{},{}", s, i),
        Language::C(s, i) => println!("{},{}", s, i),
        _ => println!("none"),
    }

    enum Color {
        Red,
        Blue,
        Green,
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    let color = Color::RGB(122, 17, 40);
    println!("What color is it?");
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
            c, m, y, k
        ),
    }
}

#[test]
fn pointers() {
    let reference = &4;
    // reference 地址
    // *reference 值
    match reference {
        &val => println!("i32 {:?} {:?}", val, &val),
    }

    match reference {
        val => println!("&i32 {:?} {:?}", val, &val),
    }

    match *reference {
        val => println!("i32 {:?} {:?}", val, &val),
    }

    match *reference {
        ref val => println!("&i32 {:?} {:?}", val, &val),
    }

    // What if you don't start with a reference? `reference` was a `&`
    // because the right side was already a reference. This is not
    // a reference because the right side is not one.
    let _not_a_reference = 3;

    // Rust provides `ref` for exactly this purpose. It modifies the
    // assignment so that a reference is created for the element; this
    // reference is assigned.
    let ref is_a_reference = 3;
    println!("{}", is_a_reference);
    // Accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`.
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly.
    #[allow(unreachable_patterns)]
    match mut_value {
        a => println!("{}", a),
        ref a => println!("{}", a),
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }
}

struct Foo {
    x: (u32, u32),
    y: u32,
}

fn struct_match(f: Foo) {
    match f {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
    }
}

#[test]
fn struct_test() {
    struct_match(Foo { x: (1, 2), y: 3 });
    struct_match(Foo { x: (0, 0), y: 3 });
    struct_match(Foo { y: 2, x: (0, 0) });
}

fn guard_match(pair: (i8, i8)) {
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // The ^ `if condition` part is a guard
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}

#[test]
fn guard() {
    guard_match((1, 2));
    guard_match((1, 1));
    guard_match((-1, 1));
    guard_match((0, 1));
}
