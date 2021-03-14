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
            assert_eq!(right, [1, 2, 3, 4, 5, 6]);
        }

        {
            let (left, right) = v.split_at(2);
            assert_eq!(left, [1, 2]);
            assert_eq!(right, [3, 4, 5, 6]);
        }

        {
            let (left, right) = v.split_at(6);
            assert_eq!(left, [1, 2, 3, 4, 5, 6]);
            assert!(right == []);
        }
    }

    #[test]
    fn split_at_mut() {
        let mut v = [1, 0, 3, 0, 5, 6];
        // scoped to restrict the lifetime of the borrows
        {
            let (left, right) = v.split_at_mut(2);
            assert_eq!(left, [1, 0]);
            assert_eq!(right, [3, 0, 5, 6]);
            left[1] = 2;
            right[1] = 4;
        }
        assert_eq!(v, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn split() {
        let slice = [10, 40, 33, 20];
        let mut iter = slice.split(|num| num % 3 == 0);

        assert_eq!(iter.next().unwrap(), &[10, 40]);
        assert_eq!(iter.next().unwrap(), &[20]);
        assert!(iter.next().is_none());

        let slice = [10, 40, 33];
        let mut iter = slice.split(|num| num % 3 == 0);

        assert_eq!(iter.next().unwrap(), &[10, 40]);
        assert_eq!(iter.next().unwrap(), &[]);
        assert!(iter.next().is_none());

        let slice = [10, 6, 33, 20];
        let mut iter = slice.split(|num| num % 3 == 0);

        assert_eq!(iter.next().unwrap(), &[10]);
        assert_eq!(iter.next().unwrap(), &[]);
        assert_eq!(iter.next().unwrap(), &[20]);
        assert!(iter.next().is_none());

        let slice = [10, 6, 9, 33, 20];
        let iter = slice.split(|num| num % 3 == 0);
        println!("{:?}", iter);
        for i in iter {
            println!("{:?}", i);
        }
    }

    #[test]
    fn split_mut() {
        let mut v = [10, 40, 30, 20, 60, 50];

        for group in v.split_mut(|num| *num % 3 == 0) {
            group[0] = 1;
        }
        assert_eq!(v, [1, 40, 30, 1, 60, 1]);
    }

    // #[test]
    // #[feature(split_inclusive)]
    // fn split_inclusive() {
    //     let slice = [10, 40, 33, 20];
    //     let mut iter = slice.split_inclusive(|num| num % 3 == 0);
    //
    //     assert_eq!(iter.next().unwrap(), &[10, 40, 33]);
    //     assert_eq!(iter.next().unwrap(), &[20]);
    //     assert!(iter.next().is_none());
    // }

    #[test]
    fn rsplit() {
        let slice = [11, 22, 33, 0, 44, 55];
        let mut iter = slice.rsplit(|num| *num == 0);

        assert_eq!(iter.next().unwrap(), &[44, 55]);
        assert_eq!(iter.next().unwrap(), &[11, 22, 33]);
        assert_eq!(iter.next(), None);

        let v = &[0, 1, 1, 2, 3, 5, 8];
        let mut it = v.rsplit(|n| *n % 2 == 0);
        assert_eq!(it.next().unwrap(), &[]);
        assert_eq!(it.next().unwrap(), &[3, 5]);
        assert_eq!(it.next().unwrap(), &[1, 1]);
        assert_eq!(it.next().unwrap(), &[]);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn rsplit_mut() {
        let mut v = [100, 400, 300, 200, 600, 500];

        let mut count = 0;
        for group in v.rsplit_mut(|num| *num % 3 == 0) {
            count += 1;
            group[0] = count;
        }
        assert_eq!(v, [3, 400, 300, 2, 600, 1]);
    }

    #[test]
    fn splitn() {
        let v = [10, 40, 30, 20, 60, 50];

        for group in v.splitn(2, |num| *num % 3 == 0) {
            println!("{:?}", group);
        }
        println!("========================================");
        for group in v.splitn(3, |num| *num % 3 == 0) {
            println!("{:?}", group);
        }
    }

    #[test]
    fn splitn_mut() {
        let mut v = [10, 40, 30, 20, 60, 50];

        for group in v.splitn_mut(2, |num| *num % 3 == 0) {
            group[0] = 1;
        }
        assert_eq!(v, [1, 40, 30, 1, 60, 50]);
    }

    #[test]
    fn rsplitn() {
        let v = [10, 40, 30, 20, 60, 50];

        for group in v.rsplitn(2, |num| *num % 3 == 0) {
            println!("{:?}", group);
        }
        println!("==========================================");
        for group in v.splitn(2, |num| *num % 3 == 0) {
            println!("{:?}", group);
        }
    }

    #[test]
    fn rsplitn_mut() {
        let mut s = [10, 40, 30, 20, 60, 50];

        for group in s.rsplitn_mut(2, |num| *num % 3 == 0) {
            group[0] = 1;
            println!("{:?}", group)
        }
        assert_eq!(s, [1, 40, 30, 20, 60, 1]);
    }

    #[test]
    fn contains() {
        let v = [10, 40, 30];
        assert!(v.contains(&30));
        assert!(!v.contains(&50));

        let v = [String::from("hello"), String::from("world")]; // slice of `String`
        assert!(v.iter().any(|e| e == "hello")); // search with `&str`
        assert!(!v.iter().any(|e| e == "hi"));
        println!("{}", v.contains(&String::from("hello")));
    }

    #[test]
    fn starts_with() {
        let v = [10, 40, 30];
        assert!(v.starts_with(&[10]));
        assert!(v.starts_with(&[10, 40]));
        assert!(!v.starts_with(&[50]));
        assert!(!v.starts_with(&[10, 50]));
        //Always returns `true` if `needle` is an empty slice:
        let v = &[10, 40, 30];
        assert!(v.starts_with(&[]));
        let v: &[u8] = &[];
        assert!(v.starts_with(&[]));
    }

    #[test]
    fn ends_with() {
        let v = [10, 40, 30];
        assert!(v.ends_with(&[30]));
        assert!(v.ends_with(&[40, 30]));
        assert!(!v.ends_with(&[50]));
        assert!(!v.ends_with(&[50, 30]));

        let v = &[10, 40, 30];
        assert!(v.ends_with(&[]));
        let v: &[u8] = &[];
        assert!(v.ends_with(&[]));
    }

    #[test]
    fn binary_search() {
        let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

        assert_eq!(s.binary_search(&13), Ok(9));
        assert_eq!(s.binary_search(&4), Err(7));
        assert_eq!(s.binary_search(&100), Err(13));
        println!("{:?}", s.binary_search(&100));
        let r = s.binary_search(&1);
        assert!(match r {
            Ok(1..=4) => true,
            _ => false,
        });

        let mut s = vec![0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        let num = 42;
        let idx = s.binary_search(&num).unwrap_or_else(|x| x);
        s.insert(idx, num);
        assert_eq!(s, [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 42, 55]);
    }

    #[test]
    fn binary_search_by() {
        let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

        let seek = 13;
        assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Ok(9));
        let seek = 4;
        assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(7));
        let seek = 100;
        assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(13));
        let seek = 1;
        let r = s.binary_search_by(|probe| probe.cmp(&seek));
        assert!(match r {
            Ok(1..=4) => true,
            _ => false,
        });
    }

    #[test]
    fn binary_search_by_key() {
        let s = [
            (0, 0),
            (2, 1),
            (4, 1),
            (5, 1),
            (3, 1),
            (1, 2),
            (2, 3),
            (4, 5),
            (5, 8),
            (3, 13),
            (1, 21),
            (2, 34),
            (4, 55),
        ];

        assert_eq!(s.binary_search_by_key(&13, |&(_, b)| b), Ok(9));
        assert_eq!(s.binary_search_by_key(&4, |&(_, b)| b), Err(7));
        assert_eq!(s.binary_search_by_key(&100, |&(_, b)| b), Err(13));
        let r = s.binary_search_by_key(&1, |&(_, b)| b);
        assert!(match r {
            Ok(1..=4) => true,
            _ => false,
        });
    }

    #[test]
    fn sort_unstable() {
        let mut v = [-5, 4, 1, -3, 2];

        v.sort_unstable();
        assert_eq!(v, [-5, -3, 1, 2, 4]);
    }

    #[test]
    fn sort_unstable_by() {
        let mut v = [5, 4, 1, 3, 2];
        v.sort_unstable_by(|a, b| a.cmp(b));
        assert_eq!(v, [1, 2, 3, 4, 5]);

        // reverse sorting
        v.sort_unstable_by(|a, b| b.cmp(a));
        assert_eq!(v, [5, 4, 3, 2, 1]);
    }

    #[test]
    fn sort_unstable_by_key() {
        let mut v = [-5i32, 4, 1, -3, 2];

        v.sort_unstable_by_key(|k| k.abs());
        assert_eq!(v, [1, 2, -3, 4, -5]);
    }
}
