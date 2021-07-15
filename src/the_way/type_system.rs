///! 深入trait
pub mod deep_trait {
    /// 抽象类型
    pub mod abstract_type {
        use std::fmt::Debug;

        #[derive(Debug)]
        struct Foo;

        trait Bar {
            fn baz(&self);
        }

        impl Bar for Foo {
            fn baz(&self) {
                println!("{:?}", self);
            }
        }

        //带trait限定的泛型函数
        fn static_dispatch<T>(t: &T)
        where
            T: Bar,
        {
            t.baz();
        }

        //trait对象
        fn dynamic_dispatch(t: &dyn Bar) {
            t.baz();
        }

        #[test]
        fn trait_object() {
            let foo = Foo;
            static_dispatch(&foo);
            dynamic_dispatch(&foo);
        }

        trait Fly {
            fn fly(&self) -> bool;
        }

        #[derive(Debug)]
        struct Duck;

        #[derive(Debug)]
        struct Pig;

        impl Fly for Duck {
            fn fly(&self) -> bool {
                true
            }
        }

        impl Fly for Pig {
            fn fly(&self) -> bool {
                false
            }
        }

        fn fly_static(s: impl Fly + Debug) -> bool {
            s.fly()
        }

        //将impl Trait语法用于返回值位置的时候，实际上等价于给返回类型增加一种trait限定范围
        fn can_fly(s: impl Fly + Debug) -> impl Fly {
            if s.fly() {
                println!("{:?} can fly", s)
            } else {
                println!("{:?} can't fly", s)
            }
            s
        }

        #[test]
        fn impl_trait() {
            let pig = Pig;
            let duck = Duck;
            assert_eq!(fly_static(pig), false);
            assert_eq!(fly_static(duck), true);
            can_fly(Pig);
            can_fly(Duck);
        }
    }
}
