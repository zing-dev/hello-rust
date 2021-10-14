// 8位无符号整型数据
#[test]
fn basic_test() {

    // const
    assert_eq!(u8::MAX, 0xff);
    assert_eq!(u8::MIN, 0x00);
    assert_eq!(u8::BITS, 8);
}

/// 根据进制将字符串转化成无符号整型数字
/// ```
/// pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError>
/// ```
#[test]
fn from_str_radix_test() {
    assert_eq!(u8::from_str_radix("A", 16), Ok(10));
    assert_eq!(u8::from_str_radix("ab", 16), Ok(0xab));
    assert_eq!(u8::from_str_radix("1000000", 2), Ok(0b1000000));

    if let Err(e) = i32::from_str_radix("a12", 10) {
        println!("Failed conversion to i32: {}", e);
        assert_eq!(e.kind(), &std::num::IntErrorKind::InvalidDigit)
    }
    if let Err(e) = i32::from_str_radix("0xa12", 16) {
        println!("Failed conversion to i32: {}", e);
        assert_eq!(e.kind(), &std::num::IntErrorKind::InvalidDigit)
    }
}

/// u8的二进制形式1或0的个数,0和1的总数必须为8
/// ```
/// pub const fn count_ones(self) -> u32
/// pub const fn count_zeros(self) -> u32
/// ```
#[test]
fn count_0_or_1_test() {
    assert_eq!(0b1100u8.count_ones(), 2);
    assert_eq!(0b1100u8.count_zeros(), 6);
    assert_eq!(110u8.count_ones(), 5);
    assert_eq!(110u8.count_zeros(), 3);
}

/// 二进制形式前导或后导的0或1的个数
/// ```
/// pub const fn leading_zeros(self) -> u32
/// pub const fn trailing_zeros(self) -> u32
/// pub const fn leading_ones(self) -> u32
/// pub const fn trailing_ones(self) -> u32
/// ```
#[test]
fn leading_zeros_or_once_test() {
    assert_eq!(u8::MIN.leading_zeros(), 8);
    assert_eq!(u8::MIN.leading_ones(), 0);
    assert_eq!(u8::MAX.leading_zeros(), 0);
    assert_eq!(u8::MAX.leading_ones(), u8::BITS);
    assert_eq!((u8::MAX & 0b00001111).leading_zeros(), 4);
    assert_eq!((u8::MAX & 0b11110000).leading_ones(), 4);

    assert_eq!(u8::MIN.trailing_zeros(), 8);
    assert_eq!(u8::MIN.trailing_ones(), 0);
    assert_eq!(u8::MAX.trailing_zeros(), 0);
    assert_eq!(u8::MAX.trailing_ones(), u8::BITS);
    assert_eq!((u8::MAX & 0b00001111).trailing_zeros(), 0);
    assert_eq!((u8::MAX & 0b11110000).trailing_ones(), 0);
}

/// ```
/// #[must_use = "this returns the result of the operation, \ without modifying the original"]
/// 将位向左移动指定的量n，将截断的位包装到结果整数的末尾。
/// pub const fn rotate_left(self, n: u32) -> u8
/// 将位向左移动指定的量n，将截断的位包装到结果整数的末尾。
/// pub const fn rotate_right(self, n: u32) -> u8
/// ```
#[test]
fn rotate_left_or_right_test() {
    // 1000 0010 << 2 0000 1010 => a
    assert_eq!(0x82u8.rotate_left(2), 0xa);
    // 1000 0010 >> 2 1010 0000 => a0
    assert_eq!(0x82u8.rotate_right(2), 0xa0);
}
