pub mod char_test {

    #[test]
    fn main() {
        let tao = '道';
        let tao_u32 = tao as u32;
        println!("{} {} {}", tao, tao_u32, tao.len_utf8()); //道 36947 3
        println!("U+{}", tao_u32);
        println!("{}", tao.escape_unicode());
        assert_eq!(char::from(65), 'A');
        assert_eq!(std::char::from_u32(tao_u32), Some(tao));
    }

    #[test]
    fn common() {
        let c = '🤣';
        println!("{}", c);
        println!("{}", c as u16); //63779
    }

    #[test]
    fn from_u32() {
        use std::char;
        let c = char::from_u32(0x2764);
        println!("{}", c.unwrap());
    }

    #[test]
    fn form_digit() {
        use std::char;

        let c = char::from_digit(4, 8);
        println!("{}", c.unwrap());
        if let Some(ch) = c {
            println!("{}", ch);
        }

        println!("{}", char::from_digit(4, 10).unwrap()); //4
        println!("{}", char::from_digit(11, 16).unwrap()); //b
        println!("{}", char::from_digit(20, 32).unwrap()); //k
    }

    #[test]
    fn from() {
        let u = 65 as u8;
        let ch = char::from(u);
        println!("{}", ch); //A
        let i = u32::from('😜');
        println!("{}", i);
        println!("{}", std::char::from_u32(i).unwrap());
    }

    #[test]
    fn is_digit() {
        println!("{}", '1'.is_digit(10));
        println!("{}", 'a'.is_digit(10));
        println!("{}", 'a'.is_digit(16));
    }

    #[test]
    fn to_digit() {
        println!("{}", '1'.to_digit(10).unwrap());
        println!("{}", 'f'.to_digit(16).unwrap());
        assert_eq!('1'.to_digit(8), Some(1));
        assert_eq!('f'.to_digit(8), None);
    }

    #[test]
    fn escape_unicode() {
        let unicode = '😁'.escape_unicode();
        println!("{:?}", unicode); //EscapeUnicode { c: '😁', state: Backslash, hex_digit_idx: 4 }
        println!("{}", unicode); //\u{1f601}
        for c in unicode {
            print!("{}", c); //\u{1f601}
        }
        println!();
    }

    #[test]
    fn len_utf8() {
        println!("{}", '1'.len_utf8()); //1
        println!("{}", 'h'.len_utf8()); //1
        println!("{}", '哈'.len_utf8()); //3
        println!("{}", 'あ'.len_utf8()); //3
        println!("{}", '😒'.len_utf8()); //4
    }

    #[test]
    fn len_utf16() {
        let n = 'ß'.len_utf16();
        assert_eq!(n, 1);

        let len = '💣'.len_utf16();
        assert_eq!(len, 2);
    }

    #[test]
    fn encode_utf8() {
        let mut b = [0; 2];
        let result = 'ß'.encode_utf8(&mut b);
        //println!("{:?}", b); //[195, 159]
        println!("{}", result);
        println!("{}", result.len());
    }

    #[test]
    fn encode_utf16() {
        let mut b = [0; 1];
        let result = 'ß'.encode_utf16(&mut b);
        //println!("{:?}", b); //[195, 159]
        println!("{:?}", result);
        println!("{}", result.len());
    }

    #[test]
    fn is_alphabetic() {
        //字母顺序
        assert!('a'.is_alphabetic());
        assert!('京'.is_alphabetic());

        let c = '💝';
        // love is many things, but it is not alphabetic
        assert!(!c.is_alphabetic());
    }

    #[test]
    fn is_lowercase() {
        assert!('a'.is_lowercase());
        assert!('δ'.is_lowercase());
        assert!(!'A'.is_lowercase());
        assert!(!'Δ'.is_lowercase());
    }

    #[test]
    fn is_uppercase() {
        assert!(!'a'.is_uppercase());
        assert!(!'δ'.is_uppercase());
        assert!('A'.is_uppercase());
        assert!('Δ'.is_uppercase());
    }

    #[test]
    fn is_whitespace() {
        assert!(!'a'.is_whitespace());
        assert!(' '.is_whitespace());
        assert!('\t'.is_whitespace());
        assert!(!'\0'.is_whitespace());
    }

    #[test]
    fn to_lowercase() {
        for c in 'İ'.to_lowercase() {
            print!("{}", c);
        }
        println!();
        println!("{:?}", 'A'.to_lowercase());
        println!("{:?}", 'A'.to_lowercase().to_string());
        println!("{:?}", 'İ'.to_lowercase()); //ToLowercase(Two('i', '\u{307}'))
        println!("{}", "i\u{307}");
        assert_eq!('İ'.to_lowercase().to_string(), "i\u{307}");
    }

    #[test]
    fn to_uppercase() {
        println!("{}", 'a'.to_uppercase());
        println!("{}", 'A'.to_uppercase());
        println!("{}", '张'.to_uppercase());
    }

    #[test]
    fn is_ascii() {
        //127
        for i in 0..256 {
            println!(
                "{} {} is_ascii is {}",
                i,
                i as u8 as char,
                (i as u8 as char).is_ascii()
            );
        }
    }

    #[test]
    fn to_ascii_uppercase() {
        println!("{}", 'a'.to_ascii_uppercase());
        println!("{}", '😒'.to_ascii_uppercase());
    }

    #[test]
    fn eq_ignore_ascii_case() {
        println!("{}", 'a'.eq_ignore_ascii_case(&'A'));
        println!("{}", 'a'.eq_ignore_ascii_case(&'B'));
    }

    #[test]
    fn make_ascii_uppercase() {
        let mut char = 'a';
        char.make_ascii_uppercase();
        println!("{}", char);
    }

    #[test]
    /// ASCII letters 'A' to 'Z' are mapped to 'a' to 'z'
    fn is_ascii_alphabetic() {
        println!("{}", 'a'.is_ascii_alphabetic());
        println!("{}", '['.is_ascii_alphabetic());
        println!("{}", '~'.is_ascii_alphabetic());
    }

    #[test]
    /// - U+0041 'A' ..= U+005A 'Z', or
    /// - U+0061 'a' ..= U+007A 'z', or
    /// - U+0030 '0' ..= U+0039 '9'.
    fn is_ascii_alphanumeric() {
        println!("{}", 'A'.is_ascii_alphanumeric())
    }

    #[test]
    /// U+0030 '0' ..= U+0039 '9'.
    fn is_ascii_digit() {}

    #[test]
    /// - U+0030 '0' ..= U+0039 '9', or
    /// - U+0041 'A' ..= U+0046 'F', or
    /// - U+0061 'a' ..= U+0066 'f'.
    fn is_ascii_hexdigit() {}
}
