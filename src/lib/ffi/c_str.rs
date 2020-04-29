pub mod c_str {
    mod nul_error {
        use std::ffi::{CString, NulError};

        #[test]
        fn nul_error() {
            let err: NulError = CString::new(b"f\0oo".to_vec()).unwrap_err();
            //nul byte found in provided data at position: 1
            println!("{}", err);
        }
    }

    mod from_bytes_with_nul_error {
        use std::ffi::{CStr, FromBytesWithNulError};

        #[test]
        fn from_bytes_with_nul_error() {
            let err: FromBytesWithNulError = CStr::from_bytes_with_nul(b"f\0oo").unwrap_err();
            //data provided contains an interior nul byte at byte pos 1
            println!("{}", err);
        }
    }

    mod c_string {
        use std::ffi::CString;
        use std::os::raw::c_char;

        #[test]
        fn new() {
            extern "C" {
                fn puts(s: *const c_char);
            }
            let to_print = CString::new("Hello!").expect("CString::new failed");
            println!("{:?}", to_print);
            unsafe {
                puts(to_print.as_ptr());
            }
        }

        /*
        #[test]
        fn from_vec_unchecked() {
            use std::ffi::CString;
            let raw = b"foo".to_vec();
            unsafe {
                let c_string = CString::from_vec_unchecked(raw);
                println!("{:?}", c_string)
            }
        }

        //error: linking with `link.exe` failed: exit code: 1120
        #[test]
        fn from_raw() {
            use std::ffi::CString;
            use std::os::raw::c_char;
            extern "C" {
                fn some_extern_function(s: *mut c_char);
            }
            let c_string = CString::new("Hello!").expect("CString::new failed");
            let raw = c_string.into_raw();
            unsafe {
                some_extern_function(raw);
                let c_string = CString::from_raw(raw);
                println!("{:?}", c_string)
            }
        }

        //error: linking with `link.exe` failed: exit code: 1120
        #[test]
        fn into_raw() {
            let c_string = CString::new("foo").expect("CString::new failed");
            let ptr = c_string.into_raw();
            unsafe {
                assert_eq!(b'f', *ptr as u8);
                assert_eq!(b'o', *ptr.offset(1) as u8);
                assert_eq!(b'o', *ptr.offset(2) as u8);
                assert_eq!(b'\0', *ptr.offset(3) as u8);
                // retake pointer to free memory
                let _ = CString::from_raw(ptr);
            }
        }

        //error: linking with `link.exe` failed: exit code: 1120
        #[test]
        fn into_string() {
            let valid_utf8 = vec![b'f', b'o', b'o'];
            let cstring = CString::new(valid_utf8).expect("CString::new failed");
            assert_eq!(
                cstring.into_string().expect("into_string() call failed"),
                "foo"
            );
            let invalid_utf8 = vec![b'f', 0xff, b'o', b'o'];
            let cstring = CString::new(invalid_utf8).expect("CString::new failed");
            let err = cstring
                .into_string()
                .err()
                .expect("into_string().err() failed");
            assert_eq!(err.utf8_error().valid_up_to(), 1);
        }

        #[test]
        fn into_bytes() {
            let c_string = CString::new("foo").expect("CString::new failed");
            let bytes = c_string.into_bytes();
            assert_eq!(bytes, vec![b'f', b'o', b'o']);
        }         */
    }
}
