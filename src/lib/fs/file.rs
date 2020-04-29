pub mod file {
    use std::fs::{remove_file, File};
    use std::io::Error;
    use std::{io, result};

    const FILENAME: &str = "a.txt";

    #[test]
    fn open() {
        let result = File::open(FILENAME);
        match result {
            Ok(f) => {
                println!("打开文件成功");
                let result = remove_file(FILENAME);
                match result {
                    Ok(_) => println!("删除成功"),
                    Err(err) => println!("删除失败:{}", err),
                }
            }
            Err(err) => {
                println!("打开文件失败:{}", err);
                let result = File::create(FILENAME);
                match result {
                    Ok(f) => println!("创建文件成功"),
                    Err(err) => println!("创建文件失败:{}", err),
                }
            }
        }
    }

    #[test]
    fn open2() {
        let result = File::open(FILENAME);
        if let Ok(f) = result {
            println!("打开文件成功");
            let result = remove_file(FILENAME);
            if let Ok(_) = result {
                println!("删除成功")
            } else if let Err(err) = result {
                println!("删除失败:{}", err)
            }
        } else if let Err(err) = result {
            println!("打开文件失败:{}", err);
            let result = File::create(FILENAME);
            if let Ok(_) = result {
                println!("创建文件成功")
            } else if let Err(err) = result {
                println!("创建文件失败:{}", err)
            }
        }
    }

    #[test]
    fn open3() -> io::Result<()> {
        File::open(FILENAME)?;
        Ok(())
    }

    #[test]
    fn create() {
        File::create(FILENAME).expect("create err");
    }
}
