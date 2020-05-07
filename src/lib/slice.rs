//stdlib/libcore/slice
pub mod slice {
    use std::any::Any;

    #[test]
    fn common() {
        let x = [1, 2, 3, 4, 5, 6, 7];
        println!("len {}", x.len()); //7
        println!("is_empty {}", x.is_empty()); //false
        println!("first {}", x.first().unwrap()); //1
        println!("last {}", x.last().unwrap()); //7
        println!("starts_with {}", x.starts_with(&[1])); //true
        println!("ends_with {}", x.ends_with(&[7])); //true
        println!("contains {}", x.contains(&2)); //true
        println!("get {:?}", x.get(1).unwrap()); //2
        println!("type_id {:?}", x.type_id()); //TypeId { t: 4133352612237058833 }
    }

    #[test]
    fn len() {
        let x = [1, 2, 3];
        println!("{}", x.len()) // 3
    }

    #[test]
    fn is_empty() {
        println!("{}", ([] as [u8; 0]).is_empty()); //true
    }

    #[test]
    fn first_mut() {
        let x = &mut [0, 1, 2];
        let ref mut y = [0, 1, 2];
        let z = &[0, 1, 2];
        //let &mut a = [0, 1, 2]; //expected array `[{integer}; 3]`, found `&mut _`
        println!("{}", x == y); //true
        println!("{}", x == z); //true
        println!("{:?}", x);
        if let Some(first) = x.first_mut() {
            *first = 5;
        }
        println!("{:?}", x);
        if let Some(first) = y.first_mut() {
            *first = 5;
        }
        println!("{:?}", y);

        // ^ `z` is a `&` reference, so the data it refers to cannot be borrowed as mutable
        /* if let Some(first) = z.first_mut() {
            *first = 5;
        }
        println!("{:?}", z);*/
    }
    #[test]
    fn split_first() {
        let x = [1, 2, 3, 4];
        let option = x.split_first();
        match option {
            None => println!("none"),
            //1,[2, 3, 4]
            Some((a, b)) => println!("{},{:?}", a, b),
        }
        println!("{:?}", x); //[1, 2, 3, 4]

        if let Some((a, b)) = x.split_first() {
            println!("{},{:?}", a, b);
        } else {
            println!("none");
        }
        println!("{},{:?}", &x[0], &x[1..]);
    }

    #[test]
    fn split_first_mut() {
        let mut x = [1, 2, 3, 4];
        if let Some((a, b)) = x.split_first_mut() {
            *a = 11;
            b[0] = 12;
            println!("{},{:?}", a, b); //11,[12, 3, 4]
        }
        println!("{:?}", x); //[11, 12, 3, 4]
    }

    #[test]
    fn split_last() {
        if let Some((a, b)) = [1, 2, 3].as_mut().split_last() {
            println!("{},{:?}", a, b); //3,[1, 2]
        }
    }

    #[test]
    fn reverse() {
        let mut v = [1, 2, 3];
        println!("{:?}", v); //[1, 2, 3]
        v.reverse();
        println!("{:?}", v); //[3, 2, 1]
    }

    #[test]
    fn iter() {
        let x = [1, 2, 3, 4, 5, 6];
        let mut iter = x.iter();
        while let Some(v) = iter.next() {
            println!("{}", v);
        }
    }

    #[test]
    fn iter_mut() {
        let mut x = [1, 2, 3, 4, 5, 6];
        let mut iter = x.iter_mut();
        while let Some(v) = iter.next() {
            *v *= 2;
            println!("while => {}", v);
        }
        for v in x.iter_mut() {
            *v /= 2;
            println!("for => {}", v);
        }
    }
}
