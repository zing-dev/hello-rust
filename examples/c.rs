use std::ffi::CString;
use std::os::raw::c_char;

fn main() {
    extern "C" {
        fn puts(s: *const c_char);
    }
    let to_print = CString::new("Hello!").expect("CString::new failed");
    unsafe {
        puts(to_print.as_ptr());
    }
}
