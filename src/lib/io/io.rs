pub mod write {
    use std::fs::{DirEntry, File};
    use std::io::{BufWriter, IoSlice, Write};
    use std::{fs, io};

    const FILENAME: &str = "test/foo.txt";

    #[test]
    fn create_dir() {
        let result = fs::read_dir("test");
        match result {
            Ok(_) => {}
            Err(_) => {
                fs::create_dir("test").expect("create dir test err");
            }
        }
    }

    #[test]
    fn write() -> io::Result<()> {
        let mut file = File::create(FILENAME)?;
        file.write(b"hello world")?;
        file.write("\nhello rust".as_bytes())?;
        file.write(&['\n' as u8, 'A' as u8, 'B' as u8])?;
        //file.write(['\n', 'a', 'b'])?;
        Ok(())
    }

    #[test]
    fn write_vectored() -> io::Result<()> {
        let mut file = File::create(FILENAME)?;
        let mut buf1 = ['A' as u8; 8];
        let mut buf2 = ['\n' as u8, 'B' as u8];
        let slice = IoSlice::new(&mut buf1);
        let slice2 = IoSlice::new(&mut buf2);
        file.write_vectored(&[slice, slice2])?;
        Ok(())
    }

    #[test]
    fn flush() -> io::Result<()> {
        let mut buffer = BufWriter::new(File::create(FILENAME)?);
        buffer.write(b"hello buffer\nhello world")?;
        buffer.flush()?;
        Ok(())
    }

    #[test]
    fn write_all() -> io::Result<()> {
        let mut file = File::create(FILENAME)?;
        file.write_all(b"hello rust")?;
        Ok(())
    }

    #[test]
    fn write_fmt() -> io::Result<()> {
        let mut buffer = File::create(FILENAME)?;
        // this call
        write!(buffer, "{:.*}", 2, 1.234567)?;
        // turns into this:
        buffer.write(b"\n")?;
        buffer.write_fmt(format_args!("{:.*}", 2, 1.234567))?;
        buffer.write(b"\n")?;
        buffer.write_fmt(format_args!("{}", "hello world"))?;
        Ok(())
    }
}

pub mod read {
    use std::fs::File;
    use std::io;
    use std::io::Read;

    const FILENAME: &str = "test/foo.txt";

    #[test]
    fn read() -> io::Result<()> {
        let mut f = File::open(FILENAME)?;
        let mut buffer = [0; 30];
        let n = f.read(&mut buffer[..])?;
        println!("The bytes: {:?}", &buffer[..n]);
        println!("The bytes: {}", String::from_utf8_lossy(&buffer));
        Ok(())
    }

    #[test]
    fn read_to_end() -> io::Result<()> {
        let mut f = File::open(FILENAME)?;
        let mut buffer = vec![];
        let n = f.read_to_end(&mut buffer)?;
        println!("{}", n);
        println!("{:?}", buffer);
        println!("{}", String::from_utf8(buffer).expect("error"));
        Ok(())
    }

    #[test]
    fn read_to_string() -> io::Result<()> {
        let mut f = File::open(FILENAME)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        println!("{}", buffer);
        println!("{}", buffer.len());
        Ok(())
    }

    #[test]
    fn read_exact() -> io::Result<()> {
        let mut f = File::open(FILENAME)?;
        let mut buffer = [0; 21];
        // read exactly 10 bytes
        f.read_exact(&mut buffer)?;
        println!("{:?}", buffer);
        Ok(())
    }

    #[test]
    fn by_ref() -> io::Result<()> {
        let mut f = File::open(FILENAME)?;
        let mut buffer = Vec::new();
        let mut other_buffer = Vec::new();
        {
            let reference = f.by_ref();
            // read at most 5 bytes
            reference.take(5).read_to_end(&mut buffer)?;
            println!("{:?}", buffer)
        } // drop our &mut reference so we can use f again

        // original file still usable, read the rest
        f.read_to_end(&mut other_buffer)?;
        Ok(())
    }

    #[test]
    fn bytes() -> io::Result<()> {
        let f = File::open(FILENAME)?;
        for byte in f.bytes() {
            println!("{}", byte.unwrap());
        }
        Ok(())
    }

    #[test]
    fn chain() -> io::Result<()> {
        let mut f1 = File::open(FILENAME)?;
        let mut f2 = File::open("bar.txt")?;

        let mut handle = f1.chain(f2);
        let mut buffer = String::new();

        // read the value into a String. We could use any Read method here,
        // this is just one example.
        handle.read_to_string(&mut buffer)?;
        Ok(())
    }

    #[test]
    fn take() -> io::Result<()> {
        let mut f = File::open(FILENAME)?;
        let mut buffer = [0; 5];

        // read at most five bytes
        let mut handle = f.take(5);

        handle.read(&mut buffer)?;
        Ok(())
    }
}
