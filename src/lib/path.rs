pub mod path {
    mod prefix {
        use std::ffi::OsStr;
        use std::path::Prefix::*;
        use std::path::{Component, Path, Prefix};

        fn get_path_prefix(s: &str) -> Prefix {
            let path = Path::new(s);
            match path.components().next().unwrap() {
                Component::Prefix(prefix_component) => prefix_component.kind(),
                _ => panic!(),
            }
        }

        #[test]
        fn test() {
            if cfg!(windows) {
                println!("================windows==================");
                assert_eq!(
                    Verbatim(OsStr::new("pictures")),
                    get_path_prefix(r"\\?\pictures\kittens")
                );
                assert_eq!(
                    VerbatimUNC(OsStr::new("server"), OsStr::new("share")),
                    get_path_prefix(r"\\?\UNC\server\share")
                );
                assert_eq!(VerbatimDisk(b'C'), get_path_prefix(r"\\?\c:\"));
                assert_eq!(
                    DeviceNS(OsStr::new("BrainInterface")),
                    get_path_prefix(r"\\.\BrainInterface")
                );
                assert_eq!(
                    UNC(OsStr::new("server"), OsStr::new("share")),
                    get_path_prefix(r"\\server\share")
                );
                assert_eq!(
                    Disk(b'C'),
                    get_path_prefix(r"C:\Users\Rust\Pictures\Ferris")
                );
            }
            println!("{:?}", Verbatim(OsStr::new("pictures")));
        }
    }

    mod path {
        use std::env;
        use std::ffi::{OsStr, OsString};
        use std::ops::Deref;
        use std::path::{self, Path};

        #[test]
        fn is_separator() {
            println!("{}", path::is_separator('/'));
            println!("{}", path::is_separator('\\'));
            println!("{}", path::is_separator(' '));
            println!("{}", path::is_separator('-'));
        }

        #[test]
        fn new() {
            let path = Path::new("test");
            println!("{:?}", path);
            println!("{}", path.exists());
            println!("{:?}", path.file_name().unwrap());
        }

        #[test]
        fn as_os_str() {
            assert_eq!(Path::new("test").as_os_str(), OsStr::new("test"));
        }

        #[test]
        fn to_str() {
            assert_eq!(Path::new("test").to_str(), Some("test"));
        }

        #[test]
        fn is_absolute() {
            assert_eq!(Path::new("test").is_absolute(), false);
        }

        #[test]
        fn is_relative() {
            assert_eq!(Path::new("test").is_relative(), true);
        }

        #[test]
        fn has_root() {
            assert_eq!(Path::new("test").has_root(), false);
            assert_eq!(Path::new("/test").has_root(), true);
            assert_eq!(Path::new("C:/test").has_root(), true);
        }

        #[test]
        fn parent() {
            println!("{:?}", Path::new("test").parent().unwrap());
            println!(
                "{:?}",
                Path::new("C:\\Users\\zing\\workspace\\rust\\hello-rust")
                    .parent()
                    .unwrap()
            )
        }

        #[test]
        fn ancestors() {
            let mut ancestors =
                Path::new("C:\\Users\\zing\\workspace\\rust\\hello-rust").ancestors();
            for a in ancestors {
                println!(">: {:?},{:?}", a, a.file_name().unwrap_or_default());
            }
            while let Some(p) = ancestors.next() {
                println!(">: {:?},{:?}", p, p.file_name().unwrap_or_default());
            }
        }

        #[test]
        fn strip_prefix() {
            use std::path::{Path, PathBuf};
            let path = Path::new("/test/haha/foo.txt");
            println!("{:?}", path.strip_prefix("/"));
            assert_eq!(path.strip_prefix("/"), Ok(Path::new("test/haha/foo.txt")));
            assert_eq!(path.strip_prefix("/test"), Ok(Path::new("haha/foo.txt")));
            assert_eq!(path.strip_prefix("/test/"), Ok(Path::new("haha/foo.txt")));
            assert_eq!(path.strip_prefix("/test/haha/foo.txt"), Ok(Path::new("")));
            assert_eq!(path.strip_prefix("/test/haha/foo.txt/"), Ok(Path::new("")));
            assert_eq!(path.strip_prefix("test").is_ok(), false);
            assert_eq!(path.strip_prefix("/haha").is_ok(), false);
            let prefix = PathBuf::from("/test/");
            assert_eq!(path.strip_prefix(prefix), Ok(Path::new("haha/foo.txt")));
        }
    }
}
