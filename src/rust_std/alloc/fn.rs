#[test]
//使用全局分配器分配内存
fn alloc_test() {
    use std::alloc::{alloc, dealloc, Layout};

    unsafe {
        // 初始化一个 u16 类型的布局
        let layout = Layout::new::<u16>();
        // 分配空间返回指向当前地址的指针
        let ptr = alloc(layout);

        // 给当前指针指向的内存赋值
        *(ptr as *mut u16) = 42;
        assert_eq!(*(ptr as *mut u16), 42);

        // 释放内存空间
        dealloc(ptr, layout);

        let layout = Layout::new::<String>();
        let ptr = alloc(layout);

        *(ptr as *mut String) = "hello world".to_string(); //用 String::from("hello world") 会报错
        assert_eq!(*(ptr as *mut String), "hello world");
        dealloc(ptr, layout);

        let layout = Layout::new::<bool>();
        let ptr = alloc(layout);
        *(ptr as *mut bool) = true;
        assert_eq!(*(ptr as *const bool), true);
        assert_eq!(*(ptr as *mut bool), true);
        assert_eq!(ptr as *mut bool, &mut true as *mut bool);
        assert_eq!(ptr as *const bool, &true as *const bool);
        dealloc(ptr, layout);
    }
}

#[test]
// 使用全局分配器分配零初始化内存
fn alloc_zeroed_test() {
    use std::alloc::{alloc_zeroed, dealloc, Layout};
    unsafe {
        let layout = Layout::new::<u16>();
        let ptr = alloc_zeroed(layout);

        assert_eq!(*(ptr as *mut u16), 0);

        dealloc(ptr, layout);


        let layout = Layout::new::<String>();
        let ptr = alloc_zeroed(layout);

        assert_eq!(*(ptr as *mut String), "");

        dealloc(ptr, layout);

        let layout = Layout::new::<bool>();
        let ptr = alloc_zeroed(layout);

        assert_eq!(*(ptr as *mut bool), false);

        dealloc(ptr, layout);
    }
}
