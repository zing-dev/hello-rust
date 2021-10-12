/// 获取CPU架构
#[test]
fn arch_test() {
    println!("{}", std::env::consts::ARCH);
    #[cfg(windows)]
    assert_eq!(std::env::consts::ARCH, "x86_64")
}

/// 获取动态库库扩展名
#[test]
fn dll_extension_test() {
    println!("{}", std::env::consts::DLL_EXTENSION);
    #[cfg(windows)]
    assert_eq!(std::env::consts::DLL_EXTENSION, "dll");

    #[cfg(linux)]
    assert_eq!(std::env::consts::DLL_EXTENSION, "so");
}

/// 获取动态库扩展前缀
#[test]
fn dll_prefix_test() {
    println!("{}", std::env::consts::DLL_PREFIX) // 空
}


/// 获取动态库扩展前缀
#[test]
fn dll_suffix_test() {
    println!("{}", std::env::consts::DLL_SUFFIX);

    #[cfg(windows)]
    assert_eq!(std::env::consts::DLL_SUFFIX, ".dll");

    #[cfg(linux)]
    assert_eq!(std::env::consts::DLL_SUFFIX, ".so");
}

/// 获取可执行文件的扩展名
#[test]
fn exe_extension_test() {
    #[cfg(windows)]
    assert_eq!(std::env::consts::EXE_EXTENSION, "exe");
}

/// 获取当前的操系统系列
#[test]
fn family_test() {
    #[cfg(windows)]
    assert_eq!(std::env::consts::FAMILY, "windows");

    #[cfg(linux)]
    assert_eq!(std::env::consts::FAMILY, "unix");
}

/// 获取当前的操系统名
#[test]
fn os_test() {
    #[cfg(windows)]
    assert_eq!(std::env::consts::OS, "windows");

    #[cfg(linux)]
    assert_eq!(std::env::consts::OS, "linux");
}