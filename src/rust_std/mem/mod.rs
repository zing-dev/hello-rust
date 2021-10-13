pub mod r#fn {
    use std::mem::{self, size_of, size_of_val};

    #[test]
    fn forget_test() {}

    /// 是否需要删除当前类型
    #[test]
    fn needs_drop_test() {}

    /// 给定的源值替换目标值,返回目标值
    ///```
    /// pub fn replace<T>(dest: &mut T, src: T) -> T
    ///```
    #[test]
    fn replace_test() {
        let i1 = 10;
        let mut i2 = 20;
        let i3 = mem::replace(&mut i2, i1);
        assert_eq!(i1, 10);
        assert_eq!(i2, 10);
        assert_eq!(i3, 20);

        let v1 = vec![1, 2, 3, 4];
        let mut v2: Vec<i32> = vec![];
        let v3 = mem::replace(&mut v2, v1);
        assert_eq!(v2, vec![1, 2, 3, 4]);
        assert!(v3.is_empty());
    }

    /// 交换两个参数的值
    /// ```
    /// pub fn swap<T>(x: &mut T, y: &mut T)
    /// ```
    #[test]
    fn swap_test() {
        let mut i1 = 10;
        let mut i2 = 20;
        println!("i1:{},i2:{}", i1, i2);
        mem::swap(&mut i1, &mut i2);
        println!("i1:{},i2:{}", i1, i2);

        let mut i1 = String::from("hi");
        let mut i2 = String::from("rust");
        println!("i1:{},i2:{}", i1, i2);
        mem::swap(&mut i1, &mut i2);
        println!("i1:{},i2:{}", i1, i2);

        let mut i1 = "hi";
        let mut i2 = "rust";
        println!("i1:{},i2:{}", i1, i2);
        mem::swap(&mut i1, &mut i2);
        println!("i1:{},i2:{}", i1, i2);

        let mut i1: &[u8; 4] = b"hi  ";
        let mut i2: &[u8; 4] = b"rust";
        println!("i1:{:?},i2:{:?}", i1, i2);
        mem::swap(&mut i1, &mut i2);
        println!("i1:{:?},i2:{:?}", i1, i2);
    }

    /// 通过给定类型的默认值替换当前的值并返回当前的值
    /// ```
    /// pub fn take<T>(dest: &mut T) -> T where    T: Default,
    /// ```
    #[test]
    fn take_test() {
        let mut v = vec![1, 2, 3, 4];
        let v2 = mem::take(&mut v);
        assert_eq!(v2, vec![1, 2, 3, 4]);
        assert_eq!(v, Vec::default());
        assert!(v.is_empty());
        assert_eq!(v.len(), 0);

        let mut s1 = String::from("hello world");
        let s2 = mem::take(&mut s1);
        assert_eq!(s1, "");
        assert_eq!(s2, String::from("hello world"));
    }

    /// 将一种类型的值的重新解释为另一种类型
    /// ```
    /// pub const unsafe extern "rust-intrinsic" fn transmute<T, U>(e: T) -> U
    /// ```
    #[test]
    fn transmute_test() {
        /// 将指针转换为函数指针。这对于函数指针和数据指针大小不同的机器来说是不可移植的。
        fn foo() -> i32 { 10 }
        let pointer = foo as *const ();
        let function = unsafe {
            mem::transmute::<*const (), fn() -> i32>(pointer)
        };
        assert_eq!(function(), 10);

        let raw_bytes = [0x78, 0x56, 0x34, 0x12];
        let num = unsafe { mem::transmute::<[u8; 4], u32>(raw_bytes) };
        println!("{:x}", num);
        // use `u32::from_ne_bytes` instead
        let num = u32::from_ne_bytes(raw_bytes);
        println!("{:x}", num);
        // or use `u32::from_le_bytes` or `u32::from_be_bytes` to specify the endianness
        let num = u32::from_le_bytes(raw_bytes); // little endpoint
        assert_eq!(num, 0x12345678);
        let num = u32::from_be_bytes(raw_bytes); // big endpoint
        assert_eq!(num, 0x78563412);

        if cfg!(target_endian = "little") {
            println!("小端存储");
            assert_eq!(num.to_le(), num)
        } else {
            println!("大端存储");
            assert_eq!(num.to_le(), num.swap_bytes())
        }


        // ============================================
        // Turning a pointer into a usize
        let ptr = &10;
        let ptr_num_transmute = unsafe { mem::transmute::<&i32, usize>(ptr) };
        println!("{}", ptr_num_transmute);
        // Use an `as` cast instead
        let ptr_num_cast = ptr as *const i32 as usize;
        println!("{}", ptr_num_cast);
        assert_eq!(ptr_num_transmute, ptr_num_cast);
        println!("{}", unsafe { *mem::transmute::<usize, &i32>(ptr_num_transmute) });
        assert_eq!(unsafe { *mem::transmute::<usize, &i32>(ptr_num_transmute) }, *ptr);

        // ============================================
        // Turning an &mut T into an &mut U
        let ptr = &mut 0;
        let val_transmuted = unsafe { mem::transmute::<&mut i32, &mut u32>(ptr) };

        // Now, put together `as` and re-borrowing - note the chaining of `as` `as` is not transitive
        let val_casts = unsafe { &mut *(ptr as *mut i32 as *mut u32) };
        assert_eq!(val_transmuted, val_casts);
        *ptr = 10;
        assert_eq!(val_transmuted, &mut 10);
        assert_eq!(val_transmuted, val_casts);

        // ============================================
        // Turning an &str into an &[u8]
        // this is not a good way to do this.
        let slice = unsafe { mem::transmute::<&str, &[u8]>("Rust") };
        assert_eq!(slice, &[82, 117, 115, 116]);

        // You could use `str::as_bytes`
        let slice = "Rust".as_bytes();
        assert_eq!(slice, &[82, 117, 115, 116]);

        // Or, just use a byte string, if you have control over the string literal
        assert_eq!(b"Rust", &[82, 117, 115, 116]);
    }

    /// 将src解释为具有类型&U，然后读取src而不移动包含的值
    /// 当前要转换的数据必须要和转换后的数据大小一致,否则会发生未定义的行为
    /// ```
    /// pub const unsafe fn transmute_copy<T, U>(src: &T) -> U
    /// ```
    #[test]
    fn transmute_copy_test() {
        /// https://doc.rust-lang.org/nomicon/other-reprs.html#reprpacked
        /// 强制Rust剥离任何填充，并且仅将类型与一个字节对齐。这可能会改善内存占用，但可能会有其他负面影响。
        #[repr(packed)]
        struct Foo {
            bar: u8,
            fun: u8,
        }

        let foo_array = [5, 10u8];
        println!("{}", size_of::<i32>());
        println!("{}", size_of::<Foo>());
        println!("{}", size_of_val(&foo_array));
        unsafe {
            // Copy the data from 'foo_array' and treat it as a 'Foo'
            let mut foo_struct: Foo = mem::transmute_copy(&foo_array);
            assert_eq!(foo_struct.bar, 5);
            assert_eq!(foo_struct.fun, 10);

            // Modify the copied data
            foo_struct.bar = 20;
            assert_eq!(foo_struct.bar, 20);
        }

        // The contents of 'foo_array' should not have changed
        assert_eq!(foo_array, [5, 10]);
    }

    /// 返回由全零字节模式表示的T类型的值,对引用类型使用会发生未定义行为
    #[test]
    fn zeroed_test() {
        let x: i32 = unsafe { mem::zeroed() };
        assert_eq!(x, 0)
    }
}
