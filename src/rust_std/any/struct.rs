#[test]
fn typeid_test() {
    use std::any::{Any, TypeId};

    fn is_string<T: ?Sized + Any>(_s: &T) -> bool {
        TypeId::of::<String>() == TypeId::of::<T>()
    }

    assert_eq!(is_string(&0), false);
    assert_eq!(is_string(&"cookie monster".to_string()), true);
}