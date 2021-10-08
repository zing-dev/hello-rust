pub mod system {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
    use std::thread;
    use std::time::Duration;

    struct Counter;

    static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

    // 实现自定义全局分配器
    unsafe impl GlobalAlloc for Counter {
        // 分配内存
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let ret = System.alloc(layout);
            if !ret.is_null() {
                ALLOCATED.fetch_add(layout.size(), SeqCst);
            }
            // println!("alloc");
            return ret;
        }


        //释放内存
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            System.dealloc(ptr, layout);
            ALLOCATED.fetch_sub(layout.size(), SeqCst);
            // println!("dealloc");
        }
    }

    // 标注
    #[global_allocator]
    static A: Counter = Counter;

    #[test]
    fn system_test() {
        println!("start");
        thread::sleep(Duration::from_secs(2));
        println!("allocated bytes before main: {}", ALLOCATED.load(SeqCst));
        thread::sleep(Duration::from_secs(2));
        println!("end");
    }
}
