pub mod env {
    use std::env;
    use std::path::Path;

    #[test]
    fn current_dir() {
        let path = env::current_dir().unwrap();
        println!("The current directory is {}", path.display());
    }

    #[test]
    fn set_current_dir() {
        let root = Path::new("/");
        assert!(env::set_current_dir(root).is_ok());
        println!(
            "Successfully changed working directory to {}!",
            root.display()
        );
        let path = env::current_dir().unwrap();
        println!("The current directory is {}", path.display());
    }

    #[test]
    fn vars() {
        for (key, value) in env::vars() {
            println!("{}: {}", key, value);
        }
    }

    #[test]
    fn var() {
        let key = "PATH";
        match env::var(key) {
            Ok(val) => println!("{}: {:?}", key, val),
            Err(e) => println!("couldn't interpret {}: {}", key, e),
        }
    }

    #[test]
    fn args_os() {
        println!("{}", env::args_os().count());
        println!("{}", env::args_os().len());
        println!("{:?}", env::args_os());
        for args in env::args_os() {
            println!("{:?}", args);
        }

        let mut args = env::args_os();
        println!("{:?}", args.next().unwrap());
        println!("{:?}", args.next().unwrap());
        println!("{:?}", args.next().unwrap());
    }
}
