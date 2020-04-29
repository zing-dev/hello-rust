pub mod os_string {
    use std::ffi::OsString;

    #[test]
    fn new() {
        let string = OsString::new();
        println!("{:?}", string);
    }

    #[test]
    fn from() {
        let string = OsString::from("hello world");
        println!("{:?}", string);
        println!("{:?}", string.len());
    }

    #[test]
    fn as_os_str() {
        let string = OsString::from("hello world");
        println!("{:?}", string);
        println!("{:?}", string.as_os_str());
        assert_eq!(string, string.as_os_str());
    }

    #[test]
    fn into_string() {
        let string = OsString::from("rust");
        if let Ok(string) = string.into_string() {
            println!("{}", string)
        }
    }

    #[test]
    fn push() {
        let mut string = OsString::default();
        string.push("hello ");
        string.push("world ");
        println!("{}", string.into_string().unwrap())
    }
}
