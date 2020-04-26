pub mod vec {

    #[test]
    fn new() {
        let vec: Vec<u8> = Vec::new();
        println!("{:?}", vec);
        println!("{}", vec.len());
    }

    #[test]
    fn with_capacity() {
        let vec: Vec<u8> = Vec::with_capacity(10);
        println!("capacity {}", vec.capacity());
        println!("len {}", vec.len());
    }

    #[test]
    fn reserve() {
        let mut vec: Vec<u8> = Vec::new();
        println!("capacity {}", vec.capacity());
        vec.reserve(5);
        println!("capacity {}", vec.capacity()); //5
        vec.reserve(6);
        println!("capacity {}", vec.capacity()); //10
        vec.reserve(11);
        println!("capacity {}", vec.capacity()); //20
    }

    #[test]
    fn reserve_exact() {
        let mut vec: Vec<u8> = Vec::with_capacity(10);
        println!("capacity {}", vec.capacity()); //10
        vec.reserve_exact(5);
        println!("capacity {}", vec.capacity()); //10
        vec.reserve_exact(15);
        println!("capacity {}", vec.capacity()); //15
    }

    #[test]
    fn as_slice() {
        let mut vec: Vec<u8> = Vec::with_capacity(10);
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        println!("{:?}", vec);
        let x = vec.as_slice();
        println!("{:?}", x);
        println!("{}", x.len());
        println!("{}", vec == x)
    }

    #[test]
    fn as_mut_slice() {
        let mut vec: Vec<u8> = Vec::with_capacity(10);
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        println!("{:?}", vec);
        let x = vec.as_mut_slice();
        x[0] = 10_u8;
        println!("{:?}", x);
    }

    #[test]
    fn split_off() {
        let mut vec = vec![10, 20, 30, 40, 50, 60, 70, 80, 90];
        println!("{:?}", vec); //[10, 20, 30, 40, 50, 60, 70, 80, 90]
        println!("{:?}", vec.split_off(7)); //[80, 90]
        println!("{:?}", vec); //[10, 20, 30, 40, 50, 60, 70]
        println!("{:?}", vec.split_off(3)); //[40, 50, 60, 70]
        println!("{:?}", vec); //[10, 20, 30]
        println!("{:?}", vec.split_off(2)); //[30]
        println!("{:?}", vec); //[10, 20]
    }

    #[test]
    fn drain() {
        let mut vec = vec![10, 20, 30, 40, 50, 60, 70, 80, 90];
        println!("{:?}", vec); //[10, 20, 30, 40, 50, 60, 70, 80, 90]
        println!("{:?}", vec.drain(1..=2).collect::<Vec<u8>>()); //[20, 30]
        println!("{:?}", vec); //[10, 40, 50, 60, 70, 80, 90]
        println!("{:?}", vec.drain(4..).collect::<Vec<u8>>()); //[70, 80, 90]
        println!("{:?}", vec); //[10, 40, 50, 60]
        println!("{:?}", vec.drain(..2).collect::<Vec<u8>>()); //[10, 40]
        println!("{:?}", vec); //[50, 60]
    }

    #[test]
    fn append() {
        let mut vec = vec![11, 22];
        let mut vec2 = vec![11, 22];
        vec.append(vec2.as_mut());
        vec.append(&mut vec![111, 222]);
        vec.append(&mut Vec::from("rust"));
        println!("{:?}", vec)
    }

    #[test]
    fn pop() {
        let mut vec = vec![10, 20, 30, 40, 50, 60, 70, 80, 90];
        while let Some(v) = vec.pop() {
            println!("{}", v)
        }
        println!("{}", vec.is_empty());
    }

    #[test]
    fn push() {
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        println!("{:?}", vec);
    }

    #[test]
    fn resize() {
        let mut vec = vec!["hello"];
        vec.resize(3, "world");
        println!("{:?}", vec); //["hello", "world", "world"]

        let mut vec = vec![' '];
        vec.resize(3, 'A');
        println!("{:?}", vec); //[' ', 'A', 'A']
    }

    #[test]
    fn extend_from_slice() {
        let mut vec = vec![1];
        vec.extend_from_slice(&[2, 3, 4]);
        println!("{:?}", vec) //[1, 2, 3, 4]
    }
}
