pub mod open_options {
    use std::fs::OpenOptions;

    #[test]
    fn new() {
        let mut options = OpenOptions::new();
        //OpenOptions(OpenOptions { read: false, write: false, append: false, truncate: false, create: false, create_new: false,
        // custom_flags: 0, access_mode: None, attributes: 0, share_mode: 7, security_qos_flags: 0, security_attributes: 0 })
        println!("{:?}", options);
        options.read(true);
        println!("{:?}", options);
        options.write(true);
    }
}
