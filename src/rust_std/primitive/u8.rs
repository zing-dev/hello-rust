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

/// 反转字节序如果当前为大端法则将转换为小端法,对u8类型,转换前后一致
/// pub const fn swap_bytes(self) -> u8
#[test]
fn swap_bytes_test() {
    assert_eq!(0x12u8.swap_bytes(), 0x12);
    assert_eq!(0xffu8.swap_bytes(), 0xff);
    assert_eq!(0xff00_u16.swap_bytes(), 0x00ffu16);
    assert_eq!(0xff00ff00_u32.swap_bytes(), 0x00ff00ff_u32);
}

/// 反转位,将当前的数值的二进制位数反转
/// pub const fn reverse_bits(self) -> u8
#[test]
fn reverse_bits_test() {
    assert_eq!(0b10101010u8.reverse_bits(), 0b01010101u8, "{}", 0b01010101u8);
}

#[test]
fn from_be_or_lt_test() {
    let n = 0x1Au8;

    if cfg!(target_endian = "big") {
        assert_eq!(u8::from_be(n), n)
    } else {
        assert_eq!(u8::from_be(n), n.swap_bytes())
    }

    if cfg!(target_endian = "little") {
        assert_eq!(u8::from_le(n), n)
    } else {
        assert_eq!(u8::from_le(n), n.swap_bytes())
    }
}

#[test]
fn to_be_or_lt_test() {
    let n = 0x1Au8;

    if cfg!(target_endian = "big") {
        assert_eq!(u8::to_be(n), n)
    } else {
        assert_eq!(u8::to_be(n), n.swap_bytes())
    }

    if cfg!(target_endian = "little") {
        assert_eq!(u8::to_le(n), n)
    } else {
        assert_eq!(u8::to_le(n), n.swap_bytes())
    }
}

/// 两个整数相加,相减,相乘,相除,余数.溢出返回None
/// pub const fn checked_add(self, rhs: u8) -> Option<u8>
/// pub const fn checked_sub(self, rhs: u8) -> Option<u8>
/// pub const fn checked_mul(self, rhs: u8) -> Option<u8>
/// pub const fn checked_div(self, rhs: u8) -> Option<u8>
/// pub const fn checked_rem(self, rhs: u8) -> Option<u8>
#[test]
fn checked_add_test() {
    assert_eq!(1u8.checked_add(1u8), Some(2u8));
    assert_eq!(0xffu8.checked_add(1u8), None);
    assert_eq!(0xffu8.checked_sub(1u8), Some(0xfe));
    assert_eq!(0xfeu8.checked_sub(0xffu8), None);
    assert_eq!(0x2u8.checked_mul(0x4u8), Some(0x8u8));
    assert_eq!(0x2u8.checked_div(0x4u8), Some(0x0u8));
    assert_eq!(0x10u8.checked_rem(0x3u8), Some(0x1u8));
}