#[cfg(test)]
#[allow(unused_variables)]
pub mod fs {
    use std::any::Any;
    use std::fs;
    use std::fs::{File, OpenOptions};
    use std::io::prelude::*;
    use std::io::BufReader;

    #[test]
    fn write_all() -> std::io::Result<()> {
        let mut file = File::create("foo.txt")?;
        file.write_all(b"Hello, world!")?;
        Ok(())
    }

    #[test]
    fn read_to_string() -> std::io::Result<()> {
        let mut file = File::open("foo.txt")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        println!("{}", contents);
        assert_eq!(contents, "Hello, world!");
        Ok(())
    }

    #[test]
    fn open() -> std::io::Result<()> {
        let file = File::open("foo.txt")?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        assert_eq!(contents, "Hello, world!");
        Ok(())
    }

    #[test]
    fn create() -> std::io::Result<()> {
        let f = File::create("foo.txt")?;
        Ok(())
    }

    #[test]
    fn file() {
        let file = OpenOptions::new().read(true).open("foo.txt");

        let mut contents = String::new();
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("foo.txt")
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        println!("{}", contents);
        println!("{}", file);
        assert_eq!(contents.len(), file)
    }

    #[test]
    fn write() -> std::io::Result<()> {
        fs::write("foo1.txt", b"Lorem ipsum")?;
        fs::write("foo2.txt", "dolor sit")?;
        Ok(())
    }

    #[test]
    fn sync_all() -> std::io::Result<()> {
        let mut f = File::create("foo.txt")?;
        f.write_all(b"Hello, world!")?;
        f.sync_all()?;
        Ok(())
    }

    #[test]
    fn sync_data() -> std::io::Result<()> {
        let mut f = File::create("foo.txt")?;
        f.write_all(b"Hello, world!")?;
        f.sync_data()?;
        Ok(())
    }

    fn set_len() -> std::io::Result<()> {
        let f = File::create("foo.txt")?;
        f.set_len(100)?;
        Ok(())
    }

    #[test]
    fn metadata() -> std::io::Result<()> {
        let f = File::open("foo.txt")?;
        let metadata = f.metadata()?;
        assert_eq!(metadata.is_dir(), false);
        assert_eq!(metadata.is_file(), true);
        println!("{}", metadata.len());
        println!("{:?}", metadata.accessed());
        println!("{:?}", metadata.created());
        println!("{:?}", metadata.modified());
        println!("{:?}", metadata.permissions());
        println!("{:?}", metadata.file_type());
        println!("{:?}", metadata.file_type().is_file());
        println!("{:?}", metadata.type_id());
        Ok(())
    }

    #[test]
    fn try_clone() -> std::io::Result<()> {
        let file = File::open("foo.txt")?;
        let file_copy = file.try_clone()?;
        println!("{:?}", file_copy);
        Ok(())
    }

    #[test]
    fn set_permissions() -> std::io::Result<()> {
        use std::fs::File;
        let file = File::open("foo.txt")?;
        let perms = file.metadata()?.permissions();
        //perms.set_readonly(false);
        //file.set_permissions(perms)?;
        Ok(())
    }

    //OpenOptions
    pub mod open_options {
        use super::*;

        #[test]
        fn new() {
            let mut options = OpenOptions::new();
            let file = options.read(true).open("foo.txt");
        }

        #[test]
        fn read() {
            let file = OpenOptions::new().read(true).open("foo.txt");
        }

        #[test]
        fn write() {
            let file = OpenOptions::new().write(true).open("foo.txt");
        }

        #[test]
        fn append() {
            let file = OpenOptions::new().append(true).open("foo.txt");
        }

        #[test]
        fn truncate() {
            let file = OpenOptions::new().truncate(true).open("foo.txt");
        }

        #[test]
        fn create() {
            let file = OpenOptions::new().create(true).open("foo.txt");
        }

        #[test]
        fn create_new() {
            let file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open("foo.txt");
        }
    }

    pub mod metadata {
        use super::*;

        #[test]
        fn file_type() -> std::io::Result<()> {
            use std::fs;
            let metadata = fs::metadata("foo.txt")?;
            println!("{:?}", metadata.file_type());
            Ok(())
        }

        #[test]
        fn is_dir() -> std::io::Result<()> {
            let metadata = fs::metadata("foo.txt")?;
            assert!(!metadata.is_dir());
            Ok(())
        }

        #[test]
        fn is_file() -> std::io::Result<()> {
            let metadata = fs::metadata("foo.txt")?;
            assert!(metadata.is_file());
            Ok(())
        }

        #[test]
        fn len() -> std::io::Result<()> {
            let metadata = fs::metadata("foo.txt")?;
            println!("{}", metadata.len());
            Ok(())
        }

        #[test]
        fn permissions() -> std::io::Result<()> {
            let metadata = fs::metadata("foo.txt")?;
            assert!(!metadata.permissions().readonly());
            Ok(())
        }

        #[test]
        fn modified() -> std::io::Result<()> {
            let metadata = fs::metadata("foo.txt")?;
            if let Ok(time) = metadata.modified() {
                println!("{:?}", time);
            } else {
                println!("Not supported on this platform");
            }
            Ok(())
        }

        #[test]
        fn metadata() -> std::io::Result<()> {
            let metadata = fs::metadata("foo.txt")?;
            if let Ok(time) = metadata.accessed() {
                println!("{:?}", time);
            } else {
                println!("Not supported on this platform");
            }
            Ok(())
        }

        #[test]
        fn created() -> std::io::Result<()> {
            let metadata = fs::metadata("foo.txt")?;
            if let Ok(time) = metadata.created() {
                println!("{:?}", time);
            } else {
                println!("Not supported on this platform");
            }
            Ok(())
        }
    }

    pub mod permissions {
        use super::*;

        #[test]
        fn readonly() -> std::io::Result<()> {
            let f = File::create("foo.txt")?;
            let metadata = f.metadata()?;
            assert_eq!(false, metadata.permissions().readonly());
            Ok(())
        }

        #[test]
        fn set_readonly() -> std::io::Result<()> {
            let f = File::create("foo.txt")?;
            let metadata = f.metadata()?;
            let mut permissions = metadata.permissions();
            permissions.set_readonly(true);
            // filesystem doesn't change
            assert_eq!(false, metadata.permissions().readonly());
            // just this particular `permissions`.
            assert_eq!(true, permissions.readonly());
            Ok(())
        }
    }

    pub mod filetype {
        use super::*;

        #[test]
        fn is_dir() -> std::io::Result<()> {
            use std::fs;
            let metadata = fs::metadata("foo.txt")?;
            let file_type = metadata.file_type();
            assert_eq!(file_type.is_dir(), false);
            Ok(())
        }

        #[test]
        fn is_file() -> std::io::Result<()> {
            use std::fs;
            let metadata = fs::metadata("foo.txt")?;
            let file_type = metadata.file_type();
            assert_eq!(file_type.is_file(), true);
            Ok(())
        }

        #[test]
        fn is_symlink() -> std::io::Result<()> {
            let metadata = fs::symlink_metadata("foo.txt")?;
            let file_type = metadata.file_type();
            assert_eq!(file_type.is_symlink(), false);
            Ok(())
        }
    }

    pub mod dir_entry {
        use super::*;

        #[test]
        fn path() -> std::io::Result<()> {
            for entry in fs::read_dir(".")? {
                let dir = entry?;
                println!("{:?}", dir.path());
            }
            Ok(())
        }

        #[test]
        fn metadata() {
            if let Ok(entries) = fs::read_dir(".") {
                for entry in entries {
                    if let Ok(entry) = entry {
                        // Here, `entry` is a `DirEntry`.
                        if let Ok(metadata) = entry.metadata() {
                            // Now let's show our entry's permissions!
                            println!("{:?}: {:?}", entry.path(), metadata.permissions());
                        } else {
                            println!("Couldn't get metadata for {:?}", entry.path());
                        }
                    }
                }
            }
        }

        #[test]
        fn file_type() {
            if let Ok(entries) = fs::read_dir(".") {
                for entry in entries {
                    if let Ok(entry) = entry {
                        // Here, `entry` is a `DirEntry`.
                        if let Ok(file_type) = entry.file_type() {
                            // Now let's show our entry's file type!
                            println!("{:?}: {:?}", entry.path(), file_type);
                        } else {
                            println!("Couldn't get file type for {:?}", entry.path());
                        }
                    }
                }
            }
        }

        #[test]
        fn file_name() {
            if let Ok(entries) = fs::read_dir(".") {
                for entry in entries {
                    if let Ok(entry) = entry {
                        // Here, `entry` is a `DirEntry`.
                        println!("{:?}", entry.file_name());
                    }
                }
            }
        }
    }

    #[test]
    #[allow(unused_must_use)]
    fn remove_file() -> std::io::Result<()> {
        fs::write("a.txt", "test");
        fs::remove_file("a.txt")?;
        Ok(())
    }

    #[test]
    fn metadata2() -> std::io::Result<()> {
        let attr = fs::metadata("/some/file/path.txt")?;
        // inspect attr ...
        Ok(())
    }

    #[test]
    fn symlink_metadata() -> std::io::Result<()> {
        let attr = fs::symlink_metadata("symlink.txt")?;
        // inspect attr ...
        Ok(())
    }

    #[test]
    fn rename() -> std::io::Result<()> {
        fs::rename("foo.txt", "b.txt")?; // Rename a.txt to b.txt
        fs::rename("b.txt", "foo.txt")?; // Rename a.txt to b.txt
        Ok(())
    }

    #[test]
    fn copy() -> std::io::Result<()> {
        fs::copy("foo.txt", "bar.txt")?; // Copy foo.txt to bar.txt
        Ok(())
    }

    #[test]
    fn hard_link() -> std::io::Result<()> {
        fs::hard_link("foo.txt", "foo2.txt")?; // Hard link a.txt to b.txt
        Ok(())
    }

    #[test]
    fn create_dir() -> std::io::Result<()> {
        fs::create_dir("test")?;
        Ok(())
    }

    fn create_dir_all() -> std::io::Result<()> {
        fs::create_dir_all("test/dir")?;
        Ok(())
    }

    #[test]
    fn remove_dir() -> std::io::Result<()> {
        fs::remove_dir("test/dir")?;
        Ok(())
    }

    #[test]
    fn remove_dir_all() -> std::io::Result<()> {
        fs::remove_dir_all("test")?;
        Ok(())
    }
}
