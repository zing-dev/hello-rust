mod fs {
    use std::fs::File;
    use std::{fs, io};

    #[test]
    fn create_dir() {
        let dir = fs::create_dir("test");
        match dir {
            Ok(_) => println!("create dir test success!"),
            Err(err) => println!("create dir test error:{}", err),
            //create dir test error:Cannot create a file when that file already exists. (os error 183)
        }
    }

    #[test]
    fn create_dir_all() {
        let dirs = fs::create_dir_all("test/dir/dir");
        match dirs {
            Ok(_) => println!("create dirs test/dir/dir success!"),
            Err(err) => println!("create dirs test/dir/dir error:{}", err),
        }
    }

    #[test]
    fn rename() {
        let result = fs::rename("test/dir/dir", "test/dir/new");
        match result {
            Ok(_) => println!("rename dirs test/dir/dir success!"),
            Err(err) => println!("rename dirs test/dir/dir error:{}", err),
        }
    }

    #[test]
    fn copy() {
        let result = fs::copy("a.txt", "test/dir/new/a.txt");
        match result {
            Ok(_) => println!("copy a.txt success!"),
            Err(err) => println!("copy a.txt error:{}", err),
        }
    }

    #[test]
    fn remove_dir() -> io::Result<()> {
        fs::create_dir("test/new2")?;
        let result = fs::remove_dir("test/new2");
        match result {
            Ok(_) => println!("remove_dir test/new2 success!"),
            Err(err) => println!("remove_dir test/new2 error:{}", err),
        };
        Ok(())
    }

    #[test]
    fn remove_dir_all() -> io::Result<()> {
        fs::create_dir_all("/test/test/test")?;
        fs::remove_dir_all("/test/test/test")?;
        Ok(())
    }

    #[test]
    fn remove_file() -> io::Result<()> {
        fs::remove_file("a.txt")?;
        Ok(())
    }

    #[test]
    fn hard_link() -> io::Result<()> {
        File::create("test/a.txt")?;
        fs::hard_link("test/a.txt", "test/b.txt")?;
        Ok(())
    }
}
