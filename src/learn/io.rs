#[cfg(test)]
pub mod io {
    use std::io;
    use std::io::{Read, Write};

    #[test]
    fn copy() -> io::Result<()> {
        let mut reader: &[u8] = b"hello";
        let mut writer: Vec<u8> = vec![];
        io::copy(&mut reader, &mut writer)?;
        assert_eq!(&b"hello"[..], &writer[..]);
        println!("{}", String::from_utf8_lossy(&writer));

        let mut r = io::repeat(0).take(4);
        let mut w = io::sink();
        assert_eq!(io::copy(&mut r, &mut w).unwrap(), 4);
        let mut r = io::repeat(0).take(1 << 17);
        //assert_eq!(io::copy(&mut r , &mut w ).unwrap(), 1 << 17);
        assert_eq!(io::copy(&mut r as &mut dyn Read, &mut w as &mut dyn Write).unwrap(), 1 << 17);
        Ok(())
    }

    #[test]
    fn empty() {
        let mut buffer = String::new();
        io::empty().read_to_string(&mut buffer).unwrap();
        assert!(buffer.is_empty());
        let mut buffer2 = String::from("hello world");
    }

    #[test]
    fn repeat() {
        let mut buffer = [0; 3];
        let mut buffer2 = [0; 13];
        println!("{:?}", buffer);
        let mut repeat = io::repeat(0b101);
        repeat.read_exact(&mut buffer).unwrap();
        repeat.read_exact(&mut buffer2).unwrap();
        println!("{:?}", buffer);
        println!("{:?}", buffer2);
        assert_eq!(buffer, [0b101, 0b101, 0b101]);
    }

    #[test]
    fn sink() {
        let buffer = vec![1, 2, 3, 5, 8];
        let num_bytes = io::sink().write(&buffer).unwrap();
        assert_eq!(num_bytes, 5);
        println!("{}", io::sink().write(&[]).unwrap())

        let mut s = io::sink();
        assert_eq!(s.write(&[]).unwrap(), 0);
        assert_eq!(s.write(&[0]).unwrap(), 1);
        assert_eq!(s.write(&[0; 1024]).unwrap(), 1024);
        assert_eq!(s.by_ref().write(&[0; 1024]).unwrap(), 1024);
    }
}
