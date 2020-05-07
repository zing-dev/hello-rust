pub mod open_options {
    use std::fs::OpenOptions;
    use std::io::{Read, Seek, SeekFrom, Write};

    #[test]
    fn new() {
        let mut options = OpenOptions::new();
        //OpenOptions(OpenOptions { read: false, write: false, append: false, truncate: false, create: false, create_new: false,
        // custom_flags: 0, access_mode: None, attributes: 0, share_mode: 7, security_qos_flags: 0, security_attributes: 0 })
        println!("{:?}", options);
        options.read(true);
        println!("{:?}", options);
        options.write(true);
    }

    #[test]
    fn open() {
        if let Ok(mut f) = OpenOptions::new()
            .truncate(true)
            .write(true)
            .read(true)
            .open("a.txt")
        {
            println!("open success!");
            if let Ok(size) = f.write("hello world".as_bytes()) {
                println!("size : {}", size);
                let mut buf = [0; 256];
                match f.seek(SeekFrom::Start(0)) {
                    Ok(_) => {}
                    Err(_) => {}
                }
                if let Ok(size) = f.read(&mut buf) {
                    println!("size : {}", size);
                    println!("{:?}", &buf[0..size]);
                    println!(
                        "content : {:?}",
                        buf.iter().filter(|&&i| i != 0).collect::<Vec<&u8>>()
                    );
                    let str = String::from_utf8_lossy(&buf).to_string();
                    println!("{}", str)
                }
            }
        }
    }
}
