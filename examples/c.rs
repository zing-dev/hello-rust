use std::ffi::CString;
use std::os::raw::{c_char, c_int};

fn main() {
    extern "C" {
        fn puts(s: *const c_char);
        fn putchar(s: *const c_int);
        fn getchar() -> c_int;
    }
    let to_print = CString::new("Hello!").expect("CString::new failed");
    const I: i32 = 72;
    unsafe {
        puts(to_print.as_ptr());
        putchar(&I);
        putchar(&(77));
        //let i = getchar();
        //println!("=> {} <=", i);
        //putchar(&i)
    }
}
