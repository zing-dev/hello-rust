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

    #[test]
    fn windows() {
        let slice = ['f', 'o', 'o'];
        let mut iter = slice.windows(4);
        println!("{:?}", iter);
        println!("{:?}", iter.len());
        assert!(iter.next().is_none());

        let slice = ['r', 'u', 's', 't'];
        let mut iter = slice.windows(2);
        assert_eq!(iter.next().unwrap(), &['r', 'u']);
        assert_eq!(iter.next().unwrap(), &['u', 's']);
        assert_eq!(iter.next().unwrap(), &['s', 't']);
        assert!(iter.next().is_none());
    }

    #[test]
    fn chunks() {
        let slice = ['l', 'o', 'r', 'e', 'm'];
        let mut iter = slice.chunks(2);
        assert_eq!(iter.next().unwrap(), &['l', 'o']);
        assert_eq!(iter.next().unwrap(), &['r', 'e']);
        assert_eq!(iter.next().unwrap(), &['m']);
        assert!(iter.next().is_none());
    }

    #[test]
    fn chunks_mut() {
        let v = &mut [0, 0, 0, 0, 0];
        let mut count = 1;

        for chunk in v.chunks_mut(2) {
            for elem in chunk.iter_mut() {
                *elem += count;
            }
            count += 1;
        }
        assert_eq!(v, &[1, 1, 2, 2, 3]);
    }

    #[test]
    fn chunks_exact() {
        let slice = ['l', 'o', 'r', 'e', 'm'];
        let mut iter = slice.chunks_exact(2);
        assert_eq!(iter.next().unwrap(), &['l', 'o']);
        assert_eq!(iter.next().unwrap(), &['r', 'e']);
        assert!(iter.next().is_none());
        assert_eq!(iter.remainder(), &['m']);
    }

    #[test]
    fn chunks_exact_mut() {
        let v = &mut [0, 0, 0, 0, 0];
        let mut count = 1;

        for chunk in v.chunks_exact_mut(2) {
            for elem in chunk.iter_mut() {
                *elem += count;
            }
            count += 1;
        }
        assert_eq!(v, &[1, 1, 2, 2, 0]);
    }

    #[test]
    fn rchunks() {
        let slice = ['l', 'o', 'r', 'e', 'm'];
        let mut iter = slice.rchunks(2);
        assert_eq!(iter.next().unwrap(), &['e', 'm']);
        assert_eq!(iter.next().unwrap(), &['o', 'r']);
        assert_eq!(iter.next().unwrap(), &['l']);
        assert!(iter.next().is_none());
    }

    #[test]
    fn rchunks_mut() {
        let v = &mut [0, 0, 0, 0, 0];
        let mut count = 1;

        for chunk in v.rchunks_mut(2) {
            for elem in chunk.iter_mut() {
                *elem += count;
            }
            count += 1;
        }
        assert_eq!(v, &[3, 2, 2, 1, 1]);
    }

    #[test]
    fn rchunks_exact() {
        let slice = ['l', 'o', 'r', 'e', 'm'];
        let mut iter = slice.rchunks_exact(2);
        assert_eq!(iter.next().unwrap(), &['e', 'm']);
        assert_eq!(iter.next().unwrap(), &['o', 'r']);
        assert!(iter.next().is_none());
        assert_eq!(iter.remainder(), &['l']);
    }

    #[test]
    fn rchunks_exact_mut() {
        let v = &mut [0, 0, 0, 0, 0];
        let mut count = 1;

        for chunk in v.rchunks_exact_mut(2) {
            for elem in chunk.iter_mut() {
                *elem += count;
            }
            count += 1;
        }
        assert_eq!(v, &[0, 2, 2, 1, 1]);
    }

    #[test]
    fn split_at() {
        let v = [1, 2, 3, 4, 5, 6];

        {
            let (left, right) = v.split_at(0);
            assert!(left == []);
            assert!(right == [1, 2, 3, 4, 5, 6]);
        }

        {
            let (left, right) = v.split_at(2);
            assert!(left == [1, 2]);
            assert!(right == [3, 4, 5, 6]);
        }

        {
            let (left, right) = v.split_at(6);
            assert!(left == [1, 2, 3, 4, 5, 6]);
            assert!(right == []);
        }
    }

    #[test]
    fn split_at_mut() {
        let mut v = [1, 0, 3, 0, 5, 6];
        // scoped to restrict the lifetime of the borrows
        {
            let (left, right) = v.split_at_mut(2);
            assert!(left == [1, 0]);
            assert!(right == [3, 0, 5, 6]);
            left[1] = 2;
            right[1] = 4;
        }
        assert!(v == [1, 2, 3, 4, 5, 6]);
    }
}
