#[test]
fn test() {
    println!("success");
}
pub mod read_dir {
    use std::error::Error;
    use std::fs::DirEntry;
    use std::{fs, io};

    #[test]
    fn test() {
        let result = fs::read_dir(".");
        match result {
            Ok(mut dir) => {
                let option = dir.next();
                match option {
                    None => println!("none"),
                    Some(result) => match result {
                        Ok(entry) => {
                            let name = entry.file_name();
                            println!("{:?}", name)
                        }
                        Err(err) => println!("{}", err),
                    },
                }
            }
            Err(err) => {
                let desc = err.description();
                println!("{}", desc);
            }
        }
    }

    #[test]
    fn test2() {
        //遍历
        if let Ok(mut dir) = fs::read_dir(".") {
            while let Some(result) = dir.next() {
                if let Ok(entry) = result {
                    let name = entry.file_name();
                    println!("{:?}", name)
                }
            }
        }
    }

    #[test]
    fn test3() {
        //遍历
        if let Ok(mut dir) = fs::read_dir(".") {
            for result in dir {
                if let Ok(entry) = result {
                    let name = entry.file_name();
                    println!("{:?}", name)
                }
            }
        }
    }

    #[test]
    fn test4() -> io::Result<()> {
        for entry in fs::read_dir(".")? {
            let dir = entry?;
            println!("{:?}", dir.file_name());
        }
        Ok(())
    }
}
