#[cfg(test)]
pub mod types_test {
    #[test]
    pub fn cast() {
        let v1 = 111111.1111;
        println!("{}", v1);

        let v2: i8 = v1 as i8;
        println!("{}", v2);
    }
}

pub mod number {
    #[test]
    fn main() {
        let num = 42u32;
        println!("{}", num);

        let num: u32 = 42;
        println!("{}", num);

        let num = 0x2a;
        println!("十六进制 {:0x} {}", num, num);

        let num = 0o106;
        println!("八进制 {:0o} {}", num, num);

        let num = 0b11011011;
        println!("二进制 {:0b} {}", num, num);

        assert_eq!(b'*', 42u8);
        assert_eq!(b'\'', 39u8);
        println!("{}", 2E10);
        println!("{}", f32::INFINITY); //inf
        println!("{}", f32::NEG_INFINITY); //-inf
        println!("{}", f32::NAN); //NaN
        println!("{}", f32::MIN); //-340282350000000000000000000000000000000
        println!("{}", f32::MAX); //340282350000000000000000000000000000000
    }
}

pub mod range {
    use std::ops;

    #[test]
    fn main() {
        println!("{:?}", 1..10);
        assert_eq!(1..10, ops::Range { start: 1, end: 10 });
        println!("{:?}", 1..=10);
        assert_eq!(1..=10, ops::RangeInclusive::new(1, 10));
        println!("{}", (1..6).sum::<i32>());
        println!("{}", (1..=6).sum::<i32>());
        for i in 1..5 {
            println!("{}", i);
        }
    }
}

#[test]
fn r#slice() {
    let arr = [1, 2, 3, 4, 5];
    assert_eq!(&arr, &[1, 2, 3, 4, 5]);
    println!("{:?}", &arr[..3]);
    println!("{:?}", &arr[2..3]);
    println!("{:?}", &arr[2..]);
    println!("{:?}", &arr[..]);
    assert_eq!(&arr.len(), &5);
    println!("{}", &arr.is_empty());
    let arr = &mut [1, 2, 3];
    println!("{:?}", arr);
    arr[0] = 10;
    println!("{:?}", arr);
    let v = vec![1, 2, 3];
    println!("{:?}", v);
}

#[test]
fn r#str() {
    let str: &'static str = "hello rust";
    let ptr = str.as_ptr();
    let len = str.len();
    let s = unsafe {
        let s = std::slice::from_raw_parts(ptr, len);
        std::str::from_utf8(s)
    };
    println!("{:?}", s);
}

#[test]
fn r#ptr() {
    // *const T
    // *mut T
    let mut x = 10;
    let p = &mut x as *mut i32;
    let y = Box::new(20);
    let py = &*y as *const i32;
    unsafe {
        *p += *py;
    }
    println!("{}", x);
}
