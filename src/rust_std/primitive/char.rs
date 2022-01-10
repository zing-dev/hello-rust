#[allow(unused_imports)]
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

pub mod method {
    #[test]
    fn from_u32_test() {
        println!("{}", char::from_u32(128175).unwrap());
        assert_eq!(128175, char::from_u32(128175).unwrap() as u32);
        assert_eq!(None, char::from_u32(char::MAX as u32 + 1));

        println!("{}", char::from_u32(0x2764).unwrap());
        println!("{}", unsafe { char::from_u32_unchecked(0x2764) });
    }

    // 根据指定进制radix将数值num转换为字符形式的值
    #[test]
    fn from_digit_test() {
        println!("{:?}", char::from_digit(100, 10));
        println!("{:?}", char::from_digit(4, 10));
        assert_eq!(None, char::from_digit(10, 10));
        assert_eq!(Some('9'), char::from_digit(9, 10));
        assert_eq!(Some('a'), char::from_digit(10, 16));
    }

    /// 判断当前的字符是否是属于指定的进制
    #[test]
    fn is_digit_test() {
        assert_eq!('1'.is_digit(2), true);
        assert_eq!('1'.is_digit(3), true);
        assert_eq!('1'.is_digit(8), true);
        assert_eq!('1'.is_digit(10), true);
        assert_eq!('1'.is_digit(16), true);
        assert_eq!('a'.is_digit(10), false);
        assert_eq!('a'.is_digit(16), true);
    }

    /// 将字符根据自身所属的进制转换成数值
    // pub fn to_digit(self, radix: u32) -> Option<u32>
    // digit:
    //  0-9
    //  a-z
    //  A-Z
    #[test]
    fn to_digit_test() {
        assert_eq!('a'.to_digit(16), Some(10));
        assert_eq!('2'.to_digit(16), Some(2));
        assert_eq!('7'.to_digit(8), Some(0o7));
    }

    /// 返回当前字符的十六进制码转换的迭代器
    /// pub fn escape_unicode(self) -> EscapeUnicode
    #[test]
    fn escape_unicode_test() {
        for c in '张'.escape_unicode() {
            print!("{}", c); //\u{5f20}
        }
    }

    /// 对字符进行转义
    /// pub fn escape_debug(self) -> EscapeDebug
    #[test]
    fn escape_debug_test() {
        assert_eq!('\n'.escape_debug().to_string(), "\\n");
    }

    /// 对字符进行默认转义
    /// pub fn escape_default(self) -> EscapeDefault
    #[test]
    fn escape_default_test() {
        assert_eq!('"'.escape_default().to_string(), "\\\"");
        assert_eq!('\t'.escape_default().to_string(), "\\t");
        assert_eq!('/'.escape_default().to_string(), "/");
        assert_eq!('\\'.escape_default().to_string(), "\\\\");
    }

    /// 返回当前字符的utf8长度
    /// pub const fn len_utf8(self) -> usize
    #[test]
    fn len_utf8_test() {
        assert_eq!('a'.len_utf8(), 1);
        assert_eq!('β'.len_utf8(), 2);
        assert_eq!('张'.len_utf8(), 3);
        assert_eq!('😄'.len_utf8(), 4);
    }

    /// 返回当前字符的utf16长度
    /// pub const fn len_utf16(self) -> usize
    #[test]
    fn len_utf16_test() {
        assert_eq!('a'.len_utf16(), 1);
        assert_eq!('β'.len_utf16(), 1);
        assert_eq!('张'.len_utf16(), 1);
        assert_eq!('😄'.len_utf16(), 2);
    }

    /// 将字符作为UTF-8编码到提供的字节缓冲区中，然后返回包含编码字符的缓冲区切片
    /// pub fn encode_utf8(self, dst: &mut [u8]) -> &mut str
    #[test]
    fn encode_utf8_test() {
        let mut b = [0; 2];
        let result = 'ß'.encode_utf8(&mut b);
        assert_eq!(result, "ß");
        assert_eq!(result.len(), 2);
        println!("{:0x}", b[0] as u32 + b[1] as u32);
        println!("{:0x}", 'β' as u32);
        println!("{}", 'β'.escape_unicode());
    }

    /// pub fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16]
    #[test]
    fn encode_utf16_test() {
        let mut b = [0; 2];
        let result = '𝕊'.encode_utf16(&mut b);
        assert_eq!(result.len(), 2);
    }

    /// 判断当前字符是否具有 Alphabetic 属性
    #[test]
    fn is_alphabetic_test() {
        assert_eq!('1'.is_alphabetic(), false);
        assert_eq!('\t'.is_alphabetic(), false);
        assert_eq!(' '.is_alphabetic(), false);
        assert_eq!('a'.is_alphabetic(), true);
        assert_eq!('张'.is_alphabetic(), true);
        assert_eq!('😄'.is_alphabetic(), false);
    }

    /// 判断当前字符是不是小写字符
    /// pub fn is_lowercase(self) -> bool
    /// 判断当前字符是不是大写字符
    /// pub fn is_uppercase(self) -> bool
    #[test]
    fn is_lowercase_or_is_uppercase() {
        assert!('a'.is_lowercase());
        assert!(!'A'.is_lowercase());
        // assert!('一'.is_uppercase());
        assert!('A'.is_uppercase());
        assert!(!'a'.is_uppercase());
    }

    #[test]
    fn make_ascii_lowercase_or_uppercase_test() {
        let mut a = 'a';
        a.make_ascii_uppercase();
        assert_eq!('A', a);
        a.make_ascii_lowercase();
        assert_eq!('a', a);
    }
}