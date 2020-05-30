use std::io;

#[test]
fn stdin() {
    let mut string = String::new();
    match io::stdin().read_line(&mut string) {
        Ok(_) => {}
        Err(_) => {}
    }
    println!("{}", string);
}

pub mod stdin {
    use std::io;
    use std::io::Read;

    #[test]
    fn lock() -> io::Result<()> {
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(&mut buffer)?;
        println!("{}", buffer);
        Ok(())
    }

    #[test]
    fn read_line() {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{}", input);
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

pub mod stdout {
    use std::io;
    use std::io::Write;

    #[test]
    fn write_all() {
        io::stdout().write_all(b"hello world").unwrap();
    }

    #[test]
    fn lock() -> io::Result<()> {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(b"hello world")?;
        Ok(())
    }
}

pub mod stderr {
    use std::io;
    use std::io::Write;

    #[test]
    fn write_all() {
        io::stderr().write_all(b"hello world").unwrap();
    }

    #[test]
    fn lock() -> io::Result<()> {
        let stderr = io::stderr();
        let mut handle = stderr.lock();
        handle.write_all(b"hello world")?;
        Ok(())
    }
}
