// A fixed-size array, denoted [T; N],
// for the element type, T, and the non-negative compile-time constant size, N.
#[allow(unused_imports)]

use std::array::IntoIter;

#[test]
fn define_test() {
    let _arr = [0; 10];
    let mut arr: [i32; 10] = [0; 10];
    arr[1] = 1;
    arr[2] = 2;

    assert_eq!([1, 2], &arr[1..=2]);

    for x in arr {
        print!("{} ", x);
    }
    println!();
    for x in &arr {
        print!("{} ", x);
    }
    println!();

    let array: [i32; 3] = [0; 3];

    // This creates a slice iterator, producing references to each value.
    // #[allow(array_into_iter)]
    // for item in array.into_iter().enumerate() {
    //     let (i, x): (usize, &i32) = item;
    //     println!("array[{}] = {}", i, x);
    // }

    // The `array_into_iter` lint suggests this change for future compatibility:
    for item in array.iter().enumerate() {
        let (i, x): (usize, &i32) = item;
        println!("array[{}] = {}", i, x);
    }

    // You can explicitly iterate an array by value using
    // `IntoIterator::into_iter` or `std::array::IntoIter::new`:
    for item in IntoIterator::into_iter(array).enumerate() {
        let (i, x): (usize, i32) = item;
        println!("array[{}] = {}", i, x);
    }
}

#[test]
fn iter() {
    let array: [i32; 3] = [0; 3];

    // This iterates by reference:
    for item in array.iter() {
        let x: &i32 = item;
        println!("{}", x);
    }

    // This iterates by value:
    for item in IntoIter::new(array) {
        let x: i32 = item;
        println!("{}", x);
    }

    // This iterates by value:
    for item in array {
        let x: i32 = item;
        println!("{}", x);
    }

    // IntoIter can also start a chain.
    // This iterates by value:
    for item in IntoIter::new(array).enumerate() {
        let (i, x): (usize, i32) = item;
        println!("array[{}] = {}", i, x);
    }
}


#[allow(unused_imports)]
pub mod method {
    // #[test]
    // fn zip_test() {
    //     let x = [1, 2, 3];
    //     let y = [4, 5, 6];
    //     println!("{:?}", y.zip(x));
    // }


    use std::borrow::{Borrow, BorrowMut};

    #[test]
    fn map_test() {
        let x = [1, 2, 3];
        let y = x.map(|v| v + 1);
        assert_eq!(y, [2, 3, 4]);

        let x = [1, 2, 3];
        let mut temp = 0;
        let y = x.map(|v| {
            temp += 1;
            v * temp
        });
        assert_eq!(y, [1, 4, 9]);

        let x = ["Ferris", "Bueller's", "Day", "Off"];
        let y = x.map(|v| v.len());
        assert_eq!(y, [6, 9, 3, 3]);
    }

    #[test]
    fn as_mut_test() {
        let mut x = [1, 2, 3, 4];
        x[1] = 20;
        let x1 = x.as_mut();//可变引用,当前变量失效了
        x1[0] = 10;
        // println!("{:?} {:?}", x, x1);
        println!("{:?}", x1);
    }

    #[test]
    fn borrow_mut_test() {
        let mut x = [1, 2, 3, 4];
        x[1] = 20;
        let x1: &mut [i32; 4] = x.borrow_mut();
        // let x2: &mut [i32; 4] = x.borrow_mut(); //^ second mutable borrow occurs here
        x1[0] = 10;
        println!("{:?}", x1);

        let x2: &mut [i32; 4] = x.borrow_mut();
        x2[2] = 30;
        println!("{:?}", x2);

        let x3: &[i32] = x.borrow();
        println!("{:?}", x3);
    }

    #[test]
    fn as_ref() {
        let mut x = [1, 2, 3, 4];
        x[0] = 10;
        let x1 = x.as_ref();
        println!("{:?}", x1);
        println!("{:?}", x1.get(0).unwrap());
    }

    #[test]
    fn default_test() {
        let x: [i32; 10] = Default::default();
        println!("{:?}", x);

        let (a, b, c, d): (bool, i32, String, char) = Default::default();
        println!("{} {} {} {}", a, b, c, d)
    }

    #[test]
    fn eq_test() {
        let x = [1, 2, 3];
        println!("{}", x.eq(&[1, 2, 3]))
    }
}