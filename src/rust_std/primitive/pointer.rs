/// Raw, unsafe pointers, *const T, and *mut T.
#[test]
fn is_null_test() {
    let s = "hello world";
    let x = s as *const str;
    assert_eq!(s.as_ptr(), s as *const str as *const u8);
    assert!(!x.is_null());

    let s = String::from("");
    assert!(!(&s as *const String).is_null());
    assert!(!("h" as *const str).is_null());
    assert!(!(&true as *const bool).is_null());
}

/// 类型强转
/// ```
/// pub const fn cast<U>(self) -> *const U
/// ```
#[test]
fn cast_test() {
    let x = "hello";
    let x1 = x as *const str;
    let x2 = x1.cast::<u8>();
    let x3 = x1 as *const u8;
    unsafe {
        println!("{} {}", *x2.offset(0) as char, *x3.offset(1) as char)
    }
}

/// 返回当前指针的引用
/// ```
/// pub unsafe fn as_ref<'a>(self) -> Option<&'a T>
/// ```
#[test]
fn as_ref_test() {
    let ptr: *const u8 = &10u8 as *const u8;

    unsafe {
        if let Some(val_back) = ptr.as_ref() {
            println!("We got back the value: {}!", val_back);
        }
        // ==
        assert_eq!(&*ptr, ptr.as_ref().unwrap());
    }
}

/// 根据指针的偏移量获取值
/// ```
/// pub unsafe fn offset(self, count: isize) -> *const T
/// ```
#[test]
fn offset_test() {
    let s: &str = "123";
    let ptr: *const u8 = s.as_ptr();

    unsafe {
        println!("{}", *ptr.offset(1) as char);
        println!("{}", *ptr.offset(2) as char);
        println!("->{}<-", *ptr.offset(5));// 超出取0
    }
}

/// ```
/// pub fn wrapping_offset(self, count: isize) -> *const T
/// ```
#[test]
fn wrapping_offset_test() {
    let v = [1, 2, 3, 4, 5];
    let mut ptr = v.as_ptr();
    let step = 2;
    let end_rounded_up = ptr.wrapping_offset(6);
    // This loop prints "1, 3, 5, "
    while ptr != end_rounded_up {
        unsafe {
            print!("{}, ", *ptr);
        }
        ptr = ptr.wrapping_offset(step);
    }
}

