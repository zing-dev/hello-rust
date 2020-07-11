pub mod command {
    #[test]
    fn test() {
        use std::process::Command;

        let output = Command::new("echo")
            .arg("Hello world")
            .output()
            .expect("Failed to execute command");

        assert_eq!(b"Hello world\n", output.stdout.as_slice());
    }

    #[test]
    fn test2() {
        use std::process::Command;

        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", "echo hello-world"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
        };

        let hello = output.stdout;
        let result = String::from_utf8(hello);
        println!("{}", result.unwrap())
    }
}
