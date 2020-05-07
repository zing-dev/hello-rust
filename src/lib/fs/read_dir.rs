#[test]
fn test() {
    println!("success");
}
pub mod read_dir {
    use std::error::Error;
    use std::fs::DirEntry;
    use std::path::Path;
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
                let desc = err;
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
        if let Ok(dir) = fs::read_dir(".") {
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

    #[test]
    fn test5() -> io::Result<()> {
        fs::read_dir(".")?
            .map(|res| {
                res.map(|e| {
                    let _buf = e.path();
                    println!("{:?} {:?}", e.file_name(), e.path())
                })
            })
            .collect::<Result<Vec<_>, io::Error>>()?;
        Ok(())
    }

    fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    visit_dirs(&path, cb)?;
                } else {
                    cb(&entry);
                }
            }
        }
        Ok(())
    }

    #[test]
    fn test6() -> io::Result<()> {
        let func = |entry: &DirEntry| {
            println!("{:?},{:?}", entry.path(), entry.file_name());
        };
        visit_dirs(Path::new("."), &func)
    }

    fn call_with_one<F>(func: F) -> usize
    where
        F: Fn(usize) -> usize,
    {
        func(1)
    }

    #[test]
    fn test7() {
        let double = |x| x * 2;
        assert_eq!(call_with_one(double), 2);
    }
}
