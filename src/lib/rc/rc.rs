pub mod rc {
    use std::rc::Rc;

    #[test]
    fn test() {
        let five = Rc::new(5);
        let five2 = five.clone();
        let five3 = five.clone();
        println!("{} {} {} ", five, five2, five3);
    }

    #[test]
    fn into_raw() {
        let x = Rc::new("hello".to_owned());
        let x_ptr = Rc::into_raw(x);
        assert_eq!(
            unsafe {
                let ptr = &*x_ptr;
                println!("{}", ptr);
                &*x_ptr
            },
            "hello"
        );

        let str = "Hello world".to_owned();
        let rc = Rc::new(str);
        let raw = Rc::into_raw(rc);

        unsafe {
            let str = &*raw;
            assert_eq!(str, "Hello world")
        }
    }

    #[test]
    fn as_ptr() {
        let x = Rc::new("hello".to_owned());
        let y = Rc::clone(&x);
        let x_ptr = Rc::as_ptr(&x);
        assert_eq!(x_ptr, Rc::as_ptr(&y));
        assert_eq!(unsafe { &*x_ptr }, "hello");
    }

    #[test]
    fn from_row() {
        let x = Rc::new("hello".to_owned());
        let x_ptr = Rc::into_raw(x);

        unsafe {
            // Convert back to an `Rc` to prevent leak.
            let x = Rc::from_raw(x_ptr);
            assert_eq!(&*x, "hello");

            // Further calls to `Rc::from_raw(x_ptr)` would be memory-unsafe.
        }
    }

    #[test]
    fn downgrade() {
        let rc = Rc::new(1);
        let weak = Rc::downgrade(&rc);
        assert_eq!(1, weak.weak_count());
        assert_eq!(1, weak.strong_count());
        let weak = Rc::downgrade(&rc);
        assert_eq!(2, weak.weak_count());
        assert_eq!(1, weak.strong_count());

        let _rc1 = rc.clone();
        assert_eq!(2, weak.weak_count());
        assert_eq!(2, weak.strong_count());

        let _ = Rc::clone(&rc);
        assert_eq!(2, weak.weak_count());
        assert_eq!(2, weak.strong_count());
    }
}
