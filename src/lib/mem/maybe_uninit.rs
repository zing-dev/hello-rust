use std::mem::MaybeUninit;

unsafe fn make_vec(out: *mut Vec<i32>) {
    // `write` does not drop the old contents, which is important.
    out.write(vec![1, 2, 3]);
}

#[test]
fn test() {
    let mut v = MaybeUninit::uninit();
    unsafe { make_vec(v.as_mut_ptr()); }
    // Now we know `v` is initialized! This also makes sure the vector gets
    // properly dropped.
    let v = unsafe { v.assume_init() };
    assert_eq!(&v, &[1, 2, 3]);
}

#[test]
fn assume_init() {
    let mut x = MaybeUninit::<bool>::uninit();
    x.write(true);
    let x_init = unsafe { x.assume_init() };
    assert_eq!(x_init, true);
}