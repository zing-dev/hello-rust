#![allow(ellipsis_inclusive_range_patterns)]
#![allow(dead_code)]
#![allow(unused_imports)]

pub mod learn;
pub mod leetcode;
pub mod lib;
pub mod test;
pub mod the_way;

fn types() {
    use learn::types;
    // types::cast();
    // types::literals();
    // types::inference();
    // types::alias()
}

fn conversion() {
    use learn::conversion;

    conversion::from_into();
    conversion::try_from_into();
    conversion::to_string()
}

fn expression() {
    use learn::expression;

    expression::expression2()
}

fn flow_control() {
    use learn::flow_control;

    flow_control::if_else();
    flow_control::loop_test();
    flow_control::loop_nested_label();
    flow_control::loop_return();
    flow_control::while_test();
    flow_control::for_range();
    flow_control::for_iter();
    flow_control::for_iter_mut();
}

fn match_test() {
    use learn::match_test;
    match_test::match_test();
    match_test::match_tuples();
    match_test::match_enums();
    match_test::match_pointers();
    match_test::match_structs();
    match_test::match_guards();
    match_test::match_bindings();
    match_test::if_let();
    match_test::while_let();
}

fn function() {
    use learn::function;

    function::function();
    function::methods();
    function::closures();
    function::capture();
    function::input_parameters();
    function::input_functions();
    function::anonymity();
    function::output_parameters();
    //    function::iter_any();
}

fn function2() {
    use learn::function2;

    function2::hof();
    function2::diverging();
    function2::diverging2();
    function2::diverging3();
}

fn mod_test() {
    use learn::mod_test;

    mod_test::mod_test();
    mod_test::struct_test();
    mod_test::use_test();
    mod_test::super_test();
}

fn attribute() {
    use learn::attribute;

    attribute::dead_code();
}

fn generics() {
    use learn::generics;
    generics::gen_fn();
    generics::implement();
    generics::gen_trait();
    generics::bounds();
    generics::case_empty();
    generics::multi_bounds();
    generics::where_test();
    generics::new_types();
    generics::the_problem();
    generics::associated_types();
    generics::phantom();
    generics::case_units()
}

fn scope() {
    use learn::scope;

    scope::raii();
    scope::move_test();
    scope::mut_test();
    scope::borrow();
    scope::mutability();
    scope::freeze();
    scope::alias();
    scope::ref_test();
    scope::ref_test();
}

fn lifetimes() {
    use learn::lifetime;
    lifetime::lifetimes();
    lifetime::explicit();
    lifetime::function();
    lifetime::methods();
    lifetime::struct_test();
    lifetime::trait_test();
    lifetime::bounds();
    lifetime::coercion();
    lifetime::static_test();
}

fn box_test() {
    use learn::std_test;
    std_test::box_test()
}

fn trait_test() {
    use learn::trait_test;
    trait_test::derive();
    trait_test::dyn_test();
    trait_test::overload();
    trait_test::drop_test();
    trait_test::iter();
    trait_test::impl_trait();
    trait_test::clone();
    trait_test::disambiguating();
}

fn concurrency() {
    use learn::concurrency;
    //    concurrency::thread1();
    //    concurrency::thread2();
    //    concurrency::thread3();
    //    concurrency::thread4();
    //    concurrency::message_passing();
    //    concurrency::message_passing2();
    concurrency::message_passing3();
}

fn collection() {
    use learn::collection;
    //    collection::vector();
    //    collection::string();
    //    collection::string2();
    //    collection::string3();
    //    collection::map();
    collection::set();
}
fn net() {
    use learn::net;
    //    net::addr();
    //    net::ip();
    //    net::tcp();
}
fn main() {
    types();
}
