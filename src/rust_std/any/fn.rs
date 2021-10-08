use std::any;

#[test]
fn type_name_test() {
    println!("bool: {}", any::type_name::<bool>());
    println!("usize: {}", any::type_name::<usize>());
    println!("&str: {}", any::type_name::<&str>());
    println!("&Any: {}", any::type_name::<dyn any::Any>()); //&Any: dyn core::any::Any

    //core::option::Option<alloc::string::String>
    println!("Option<String>: {}", any::type_name::<Option<String>>());

    //core::option::Option<alloc::vec::Vec<alloc::string::String>>
    println!("Option<Vec<String>>: {}", any::type_name::<Option<Vec<String>>>());
}

// 不稳定方法
// #[test]
// fn type_name_of_val_test() {
//     use std::any::type_name_of_val;
//     let x = 1;
//     println!("{}", type_name_of_val(&x));
//     let y = 1.0;
//     println!("{}", type_name_of_val(&y));
// }