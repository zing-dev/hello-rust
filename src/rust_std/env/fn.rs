#[allow(unused_imports)]
use std::ffi::OsString;
#[allow(unused_imports)]
use std::path::Path;

/// 获取当前项目的工作目录
#[test]
fn current_dir_test() {
    let dir = std::env::current_dir();
    match dir {
        Ok(d) => {
            println!("current dir is {}", d.display());
            println!("is_dir {}", d.is_dir());
            println!("is_absolute {}", d.is_absolute());
        }
        Err(e) => {
            println!("err: {}", e);
        }
    }
}

/// 获取当前软件的执行目录
#[test]
fn current_exe_test() {
    let exe = std::env::current_exe();
    match exe {
        Ok(d) => {
            println!("current exe is {}", d.display());
            println!("is_dir {}", d.is_dir());
            println!("is_absolute {}", d.is_absolute());
        }
        Err(e) => {
            println!("err: {}", e);
        }
    }
}

/// 连接PATH环境变量
/// 参数为多个路径的迭代器
/// ```
/// pub fn join_paths<I, T>(paths: I) -> Result<OsString, JoinPathsError>
/// where
///     I: IntoIterator<Item = T>,
///     T: AsRef<OsStr>,
/// ```
#[test]
fn join_paths_test() {
    let paths = [
        Path::new("/bin"),
        Path::new("/usr/bin")
    ];
    let path_os_string = match std::env::join_paths(paths.iter()) {
        Ok(p) => p,
        Err(e) => { panic!("err {:?}", e) }
    };
    #[cfg(windows)]
        {
            assert_eq!(path_os_string, OsString::from("/bin;/usr/bin"));

            let paths = [Path::new("/bin"), Path::new("/usr/bi;;n")];
            assert!(!std::env::join_paths(paths.iter()).is_err());

            if let Some(path) = std::env::var_os("PATH") {
                let mut paths = std::env::split_paths(&path).collect::<Vec<_>>();
                // paths.push(PathBuf::from("/home/xyz/bin"));
                paths.push(std::env::current_dir().unwrap());
                let new_path = std::env::join_paths(paths).expect("join_paths err");
                std::env::set_var("PATH", &new_path);
                println!("{:?}", std::env::var_os("PATH").unwrap());
            }
        }

    #[cfg(linux)]
        {
            assert_eq!(path_os_string, OsString::from("/bin:/usr/bin"));
        }
}


/// 获取当前程序开始运行的参数
/// 返回的迭代器会检查进程的参数是否有效
/// 第一个从参数为当前程序的执行路径
#[test]
fn args_test() {
    for a in std::env::args() {
        println!("{}", a);
    }
}

/// 获取当前程序开始运行的参数,返回值为当前操作系统相关的字符串类型
/// 返回的迭代器不会检查进程的参数是否有效
#[test]
fn args_os_test() {
    for a in std::env::args_os() {
        println!("{}", a.into_string().unwrap());
    }
}

/// 删除当前进程的环境变量
#[test]
fn remove_var_test() {
    const KEY: &str = "key";
    const VAL: &str = "value";
    std::env::set_var(KEY, VAL);
    println!("{} {}", KEY, std::env::var(KEY).unwrap());
    // assert!(std::env::var(KEY).unwrap(), VAL);
    std::env::remove_var(KEY);
    println!("{} {:?}", KEY, std::env::var(KEY));
}

/// 设置新的工作目录
#[test]
fn set_current_dir_test() {
    println!("{}", std::env::current_dir().unwrap().display());
    std::env::set_current_dir(Path::new("C:/")).unwrap();
    println!("{}", std::env::current_dir().unwrap().display());
}

/// 根据环境变量的分隔符分割环境变量字符串
#[test]
fn split_paths_test() {
    let key = "path";
    match std::env::var_os(key) {
        Some(paths) => {
            for path in std::env::split_paths(&paths) {
                println!("'{}'", path.display());
            }
        }
        None => println!("{} is not defined in the environment.", key)
    }
}