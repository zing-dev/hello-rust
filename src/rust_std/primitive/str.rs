pub mod method {
    /// 将字符切片转换成字节数组
    /// ```
    /// pub const fn as_bytes(&self) -> &[u8]
    /// ```
    #[test]
    fn as_bytes_test() {
        let s: &str = "hello world";
        assert_eq!(b"hello world", s.as_bytes());
        for b in s.as_bytes() {
            println!("{1}:{0}", b, char::from(*b));
        }
    }

    /// 将字符切片转换成可变的字节切片
    /// ```
    /// pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8]
    /// ```
    #[test]
    fn as_bytes_mut_test() {
        let mut s = String::from("Hello");
        let bytes = unsafe { s.as_bytes_mut() };

        assert_eq!(b"Hello", bytes);

        let mut s = String::from("🗻∈🌏");
        println!("{}", s);
        unsafe {
            let bytes = s.as_bytes_mut();
            bytes[0] = 0xF0;
            bytes[1] = 0x9F;
            bytes[2] = 0x8D;
            bytes[3] = 0x94;

            let v = bytes.to_vec();
            let s = std::str::from_utf8(&v).unwrap();
            println!("{}", s)
        }
        assert_eq!("🍔∈🌏", s);
    }

    /// 将可变字符切片转换成原始指针
    /// ```
    ///  pub fn as_mut_ptr(&mut self) -> *mut u8
    /// ```
    #[test]
    fn as_mut_ptr_test() {
        let mut s = String::from("hello");
        let x = s.as_mut_ptr();
        println!("{:?}", x);

        unsafe {
            let parts_mut = std::slice::from_raw_parts_mut(x, s.len());
            let str = String::from_utf8(parts_mut.to_vec()).unwrap();
            println!("{}", str)
        }
    }


    /// 将字符切片转换成原始指针
    /// ```
    /// pub const fn as_ptr(&self) -> *const u8
    /// ```
    #[test]
    fn as_ptr_test() {
        let s = "hello world";
        let x = s.as_ptr();
        unsafe {
            for i in 0..s.len() {
                print!("{}", *x.offset(i as isize) as char)
            }
        }
    }
}