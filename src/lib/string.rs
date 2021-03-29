pub mod string {
    use std::borrow::Cow;
    use std::ops::Index;

    #[test]
    fn new() {
        let mut string = String::new();
        println!("{}", string);
        string.push('a');
        println!("{}", string)
    }

    #[test]
    fn with_capacity() {
        let mut string = String::with_capacity(10);
        println!("len {}", string.len()); //0
        println!("capacity {}", string.capacity()); //10

        for i in 0..string.capacity() {
            string.push((i as u8 + 65) as char);
        }
        println!("{}", string);
        println!("len {}", string.len());
        string.push('a');
        println!("len {}", string.len()); //11
        println!("len {}", string.capacity()); //20
    }

    #[test]
    fn from_utf8() {
        let result = String::from_utf8(vec![240, 159, 146, 150]);
        let string = match result {
            Ok(string) => {
                println!("match => {}", string);
                string
            }
            Err(_) => {
                println!("err");
                "".to_string()
            }
        };
        println!("{}", string)
    }

    #[test]
    fn from_utf8_lossy() {
        let sparkle_heart = vec![240, 159, 146, 150];
        let string = String::from_utf8_lossy(&sparkle_heart);
        println!("{}", string);
        match string {
            Cow::Borrowed(string) => println!("Borrowed => {}", string),
            Cow::Owned(string) => println!("Owned => {}", string),
        }

        let input = b"Hello \xF0\x90\x80World";
        let output = String::from_utf8_lossy(input);
        println!("{}", output)
    }

    #[test]
    fn into_bytes() {
        let vec = String::from("abcd").into_bytes();
        println!("{:?}", vec);
        println!("capacity : {}", vec.capacity());
        println!("len : {}", vec.len());
        println!("first : {:?}", vec.first().unwrap());
    }

    #[test]
    fn as_str() {
        let s = String::from("foo");
        println!("{}", s);
        println!("{}", s.as_str());
        println!("{}", s.as_str().len());
    }

    #[test]
    fn as_mut_str() {
        let mut string = String::from("hello rust");
        let x = string.as_mut_str();
        x.make_ascii_uppercase();
        println!("{}", string)
    }

    //追加字符串
    #[test]
    fn push_str() {
        let mut str = String::from("hello");
        str.push_str(" world");
        str.push(' ');
        str.push('🦐');
        println!("{}", str);

        //Extend 迭代器
        str.extend(['a', 'b', 'c'].iter());
        str.extend("hello".chars());
        str.extend("w o r l d".split_whitespace());
        println!("{}", str);
    }

    //插入字符串 byte position.
    #[test]
    fn insert() {
        let mut s = String::with_capacity(3);
        s.insert(0, 'f');
        s.insert(1, 'o');
        s.insert(2, 'o');
        println!("{:?}", s);
        s.insert(2, '1');
        println!("{:?}", s); //"fo1o"
        println!("{:?}", s.capacity()); //6
    }

    #[test]
    fn insert_str() {
        let mut str = String::new();
        str.insert_str(0, "hello");
        str.insert_str(str.len(), "\t");
        str.insert_str(str.len(), "world");
        println!("{}", str); //hello	world
    }

    //连接字符串
    //String 类型的字符串实现了Add<&str>和AddAssign<&str>trait
    //操作符右边的字符串为切片类型( &str )。&right 实为&String 类型,String类型实现了Deref trait ,自动解引用为&str类型 。
    #[test]
    fn join() {
        let hello = "hello".to_string();
        let world = "world".to_string();
        let mut str = hello + &world;
        println!("{}", str);
        str += "!";
        println!("{}", str);
    }

    //更新字符串
    #[test]
    fn update() {
        //使用索引来操作字符串
        let str = String::from("hello world");
        let mut res = str.clone().into_bytes();
        (0..res.len()).for_each(|i| {
            if i % 2 == 0 {
                res[i] = res[i].to_ascii_lowercase();
            } else {
                res[i] = res[i].to_ascii_uppercase();
            }
        });
        println!("{}", String::from_utf8(res).unwrap());

        //字符迭代来处理字符串;
        println!(
            "{}",
            str.chars()
                .enumerate()
                .map(|(i, c)| {
                    if i % 2 == 0 {
                        c.to_lowercase().to_string()
                    } else {
                        c.to_uppercase().to_string()
                    }
                })
                .collect::<String>()
        )
    }

    //删除字符串
    #[test]
    fn pop() {
        let mut string = String::from("hello world");
        println!("{}", string);
        println!("{:?} {}", string.pop(), string);
        println!("{:?} {}", string.pop(), string);
        while let Some(i) = string.pop() {
            if i != ' ' {
                print!("{}", i)
            }
        }
    }

    #[test]
    fn truncate() {
        let mut string = String::from("hello world");
        println!("{}", string); //hello world
        println!("len {}", string.len()); //11
        string.truncate(5);
        println!("{}", string); //hello
        println!("len {}", string.len()); //5
    }

    #[test]
    fn remove() {
        let mut string = String::from("hello world");
        while !string.is_empty() {
            println!("{} -> {}", string.clone(), string.remove(string.len() - 1));
        }
    }

    #[test]
    fn clear() {
        let mut str = String::from("Hello World");
        println!("capacity {}", str.capacity()); //11
        println!("len {}", str.len()); //11
        str.clear();
        println!("capacity {}", str.capacity()); //11
        println!("len {}", str.len()); //0
        println!("is_empty {}", str.is_empty()); //true
    }

    #[test]
    //removes the specified range in the `String` and yields the removed `chars`.
    fn drain() {
        let mut s = String::from("α is alpha, β is beta");
        let beta_offset = s.find('β').unwrap_or(s.len());
        println!("{}", beta_offset);
        // Remove the range up until the β from the string
        let t: String = s.drain(..beta_offset).collect();
        assert_eq!(t, "α is alpha, ");
        assert_eq!(s, "β is beta");

        // A full range clears the string
        s.drain(..);
        assert_eq!(s, "");

        let mut string = String::from("hello world");
        let string1 = string.drain(2..4).collect::<String>();
        println!("{}", string1);
        println!("{}", string);
    }

    #[test]
    fn reserve() {
        let mut string = String::from("hello world");
        println!("capacity {}", string.capacity()); //11
        println!("len {}", string.len()); //11
        string.reserve(8);
        println!("{}", string);
        println!("capacity {}", string.capacity()); //22
        println!("len {}", string.len()); //11
        string.reserve(15);
        println!("capacity {}", string.capacity()); //44
        println!("len {}", string.len()); //11
    }

    #[test]
    fn reserve_exact() {
        let mut string = String::from("hello world");
        println!("capacity {}", string.capacity()); //11
        println!("len {}", string.len()); //11
        string.reserve_exact(8);
        println!("{}", string);
        println!("capacity {}", string.capacity()); //19
        println!("len {}", string.len()); //11
        string.reserve_exact(15);
        println!("capacity {}", string.capacity()); //16
        println!("len {}", string.len()); //11
    }

    #[test]
    fn shrink_to_fit() {
        let mut string = String::with_capacity(40);
        println!("capacity {}", string.capacity()); // 40
        println!("len {}", string.len()); //0
        string.push_str("hello world");
        println!("capacity {}", string.capacity()); //40
        println!("len {}", string.len()); //11
        string.shrink_to_fit();
        println!("capacity {}", string.capacity()); //11
        println!("len {}", string.len()); //11
    }

    #[test]
    fn retain() {
        let mut s = String::from("abcde");
        let keep = [false, true, true, false, true];
        let mut i = 0;
        s.retain(|_| (keep[i], i += 1).0);
        println!("{}", s);
        let mut str = "rust golang".to_owned();
        str.retain(|c| c != ' ');
        println!("{}", str);
    }

    #[test]
    fn as_mut_vec() {
        let mut str = String::new();
        str.insert_str(0, "hello");
        str.insert_str(str.len(), "\t");
        str.insert_str(str.len(), "world");

        unsafe {
            let x = str.as_mut_vec();
            println!("{:?}", x);
        }
    }

    #[test]
    fn split_off() {
        let mut str = String::from("Hello World");
        let string = str.split_off(4);
        println!("{}", str); //Hell
        println!("{}", string); //o World

        let string = str.split_off(str.len());
        println!("{}", str); //Hell
        println!("{}", string); //o World
    }

    #[test]
    fn replace_range() {
        let mut s = String::from("α is alpha, β is beta");
        let beta_offset = s.find('β').unwrap_or(s.len());

        // Replace the range up until the β from the string
        s.replace_range(..beta_offset, "Α is capital alpha; ");
        assert_eq!(s, "Α is capital alpha; β is beta");
    }

    #[test]
    fn as_bytes() {
        println!("{:?}", String::from("RUST").as_bytes());
        println!("{:?}", String::from("你好").as_bytes()); //[228, 189, 160, 229, 165, 189]
    }

    #[test]
    fn into_boxed_str() {
        let str = String::from("hello world");
        let x = str.into_boxed_str();
        println!("{}", x);
        println!("{}", x.len());
        println!("{}", x.to_uppercase());
    }

    #[test]
    fn get() {
        let str = String::from("hello world");
        //println!("{}",str[0]) //`String` cannot be indexed by `{integer}`
        //println!("{}", str.index(0)); //`String` cannot be indexed by `{integer}`
        //println!("{}",str.get(0)); //string indices are ranges of `usize`
        let mut chars = str.chars();
        println!("{}", chars.nth(0).unwrap())
    }
}
