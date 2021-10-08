// A fixed-size array, denoted [T; N],
// for the element type, T, and the non-negative compile-time constant size, N.

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
    #[allow(array_into_iter)]
    for item in array.into_iter().enumerate() {
        let (i, x): (usize, &i32) = item;
        println!("array[{}] = {}", i, x);
    }

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