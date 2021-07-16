/// 深入trait
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

    mod tag_trait {
        mod send_sync {
            use std::rc::Rc;
            use std::thread;

            ///多线程共享不可变变量
            #[test]
            fn share() {
                let x = vec![1, 2, 3, 4];
                let s = thread::spawn(|| x);
                s.join().unwrap();
            }

            /// 多线程之间共享可变变量
            #[test]
            fn share_mut() {
                let mut x = vec![1, 2, 3, 4];
                thread::spawn(move || x.push(1));
                // x.push(2)
            }

            /// 在多线程之间传递没有实现Send和Sync的类型
            ///
            /// ```
            /// #[test]
            /// #![allow(dead_code)]
            /// fn share_rc() {
            ///     // Re 是用于引用计数的智能指针
            ///     let x = Rc::new(vec![1, 2, 3, 4]);
            ///     thread::spawn(move || x[1]);
            ///     // `Rc<Vec<i32>>` cannot be sent between threads safely
            /// }
            /// ```
            fn share_rc() {}
        }
    }
}

pub mod type_conversion {
    mod deref {
        use std::ops::Deref;
        use std::rc::Rc;

        /// 自动解引用
        #[test]
        fn auto() {
            let a = "hello".to_string();
            let b = " world".to_string();
            let c = a + &b;
            println!("{}", c)
        }

        #[test]
        fn auto_vec() {
            fn foo(s: &[i32]) {
                println!("{:?}", s)
            }
            let v = vec![1, 2, 3];
            foo(&v);
            foo(&Vec::new())
        }

        #[test]
        fn handle() {
            let x = Rc::new("hello");
            let y = x.clone(); // Rc<&str>
            let z = (*x).clone(); // &str
            println!("{} {} {}", x, y, z);

            let x = "hello".to_string();
            // x.deref() x.as_str() x.as_ref() &*x &x[..]
            match x.deref() {
                "hello" => println!("hello"),
                _ => (),
            }
        }
    }

    mod as_test {
        // 无歧义的完全限定语法
        mod limit {
            struct S(i32);

            trait A {
                fn test(&self, i: i32);
            }

            trait B {
                fn test(&self, i: i32);
            }

            impl A for S {
                fn test(&self, i: i32) {
                    println!("from A: {:?}", i);
                }
            }

            impl B for S {
                fn test(&self, i: i32) {
                    println!("from B: {:?}", i);
                }
            }

            #[test]
            fn test() {
                let s = S(1);
                // 直接当作trait静态函数调用
                A::test(&s, 1);
                B::test(&s, 1);
                // 使用as操作符
                <S as A>::test(&s, 1);
                <S as B>::test(&s, 1);
            }
        }

        mod static_as {
            #[test]
            fn test() {
                let a: &'static str = "hello";
                let b: &str = a as &str;
                let c: &'static str = b as &'static str;

                println!("{} {} {}", a, b, c)
            }
        }

        mod from_into {
            #[derive(Debug)]
            struct Person {
                name: String,
            }

            impl Person {
                fn new<T: Into<String>>(name: T) -> Person {
                    Person { name: name.into() }
                }
            }

            #[test]
            fn test_into() {
                let p1 = Person::new("zing");
                let p2 = Person::new("zing".to_string());
                println!("{:?} {:?}", p1, p2);
            }
        }
    }
}
