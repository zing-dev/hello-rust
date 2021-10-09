use std::mem;

#[test]
fn basic_usage_test() {
    let v = vec!['h', 'e', 'l', 'l', 'o'];

    // five elements times four bytes for each element
    assert_eq!(20, v.len() * std::mem::size_of::<char>());

    let s = String::from("hello");

    // five elements times one byte per element
    assert_eq!(5, s.len() * std::mem::size_of::<u8>());

    let mut chars = "é".chars();
    // U+00e9: 'latin small letter e with acute'
    assert_eq!(Some('\u{00e9}'), chars.next());
    assert_eq!(None, chars.next());

    let mut chars = "é".chars();
    // U+0065: 'latin small letter e'
    assert_eq!(Some('\u{0065}'), chars.next());
    // U+0301: 'combining acute accent'
    assert_eq!(Some('\u{0301}'), chars.next());
    assert_eq!(None, chars.next());

    let s = String::from("love: ❤️");
    let v: Vec<char> = s.chars().collect();

    assert_eq!(12, std::mem::size_of_val(&s[..]));
    assert_eq!(32, std::mem::size_of_val(&v[..]));
}

#[test]
#[allow(unused_parens)]
fn size_of_test() {
    println!("char {}", mem::size_of::<char>()); // 4
    println!("str {}", mem::size_of::<&str>()); // 16
    println!("String {}", mem::size_of::<String>());// 24
    println!("u8 {}", mem::size_of::<u8>()); // 1
    println!("i8 {}", mem::size_of::<i8>()); // 1
    println!("i16 {}", mem::size_of::<i16>()); // 2
    println!("i128 {}", mem::size_of::<i128>()); // 16
    println!("f32 {}", mem::size_of::<f32>()); // 4
    println!("f64 {}", mem::size_of::<f64>()); // 8
    println!("bool {}", mem::size_of::<bool>()); // 1
    println!("() {}", mem::size_of::<()>()); // 0
    println!("(bool) {}", mem::size_of::<(bool)>()); // 1
    println!("(bool,i32) {}", mem::size_of::<(bool, i32)>()); // 8
    println!("Vec<bool> {}", mem::size_of::<Vec<bool>>()); // 24

    assert_eq!(mem::size_of::<&i32>(), mem::size_of::<*const i32>());
    assert_eq!(mem::size_of::<&i32>(), mem::size_of::<Box<i32>>());
    assert_eq!(mem::size_of::<&i32>(), mem::size_of::<Option<&i32>>());
    assert_eq!(mem::size_of::<Box<i32>>(), mem::size_of::<Option<Box<i32>>>());
}

#[test]
fn const_max_test() {
    println!("{}", char::MAX);
    println!("{}", char::MAX as i32);
}