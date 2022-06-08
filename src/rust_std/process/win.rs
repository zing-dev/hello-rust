#[cfg(windows)]
pub mod process {
    use std::process::Command;
    use std::thread;
    use std::time::Duration;

    fn hello_world() {
        let output = Command::new("echo")
            .arg("Hello world")
            .output()
            .expect("Failed to execute command");

        println!("{}", String::from_utf8(output.stdout.as_slice().to_vec()).unwrap());
        assert_eq!(b"Hello world\n", output.stdout.as_slice());

        let output = Command::new("cargo").arg("run").arg("--bin=process").output().expect("execute err");
        println!("{}", String::from_utf8(output.stdout.as_slice().to_vec()).unwrap());
    }

    #[test]
    fn test_hello_world() {
        hello_world()
    }

    pub fn restart() {
        println!("current_dir {}", std::env::current_dir().unwrap().display());
        println!("current_exe {}", std::env::current_exe().unwrap().display());

        // let mut child = Command::new("cargo").arg("run").arg("--bin=process").spawn().expect("spawn err");
        // child.wait().unwrap();


        // let output = Command::new(std::env::current_exe().unwrap().display().to_string()).output().expect("execute err");
        // println!("{}",String::from_utf8(output.stdout.as_slice().to_vec()).unwrap());

        let job = thread::spawn(|| {
            let mut i = 0;
            loop {
                println!("run... {}",i);
                thread::sleep(Duration::from_secs(1));
                if i > 5 {
                    let output = Command::new("taskkill").arg("//im").arg("bin-process.exe").arg("//F").output().expect("execute err");
                    println!("taskkill: {}", String::from_utf8(output.stdout.as_slice().to_vec()).unwrap());
                    let output = Command::new(std::env::current_exe().unwrap().display().to_string()).output().expect("execute err");
                    println!("start: {}", String::from_utf8(output.stdout.as_slice().to_vec()).unwrap());
                    break
                }
                if i > 20 {
                    break;
                }
                i += 1;
            }
        });

        job.join();
    }

    #[test]
    fn test_restart() {
        restart();
    }
}