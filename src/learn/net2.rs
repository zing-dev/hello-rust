#[cfg(test)]
#[allow(unused_variables)]
mod net {
    use std::io::{BufRead, BufReader, BufWriter, Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::thread::sleep;
    use std::time::Duration;
    use std::{io, thread};

    #[test]
    fn string() {
        let sparkle_heart = vec![70];
        println!("{}", String::from_utf8(sparkle_heart).unwrap());
        println!("{}", String::from_utf8(vec![70]).unwrap());
    }

    #[test]
    #[allow(unused_must_use)]
    fn client() -> io::Result<()> {
        let mut stream = match TcpStream::connect("127.0.0.1:8080") {
            Ok(stream) => stream,
            Err(err) => panic!(err),
        };
        stream.set_read_timeout(Some(Duration::new(0, 0)))?;
        stream.set_write_timeout(Some(Duration::new(0, 0)))?;

        loop {
            let buf = [66, 67, 68, 69, 70];
            let mut buf2: [u8; 100] = [0; 100];
            stream.write(&buf)?;
            let result = match stream.read(&mut buf2) {
                Ok(length) => {
                    println!("len {}", length);
                    sleep(Duration::new(1, 0))
                }
                Err(err) => {
                    continue;
                }
            };
        }
    }

    #[test]
    pub fn server() {
        let l = TcpListener::bind("127.0.0.1:8080").unwrap();
        println!("================服务端启动=================");
        for stream in l.incoming() {
            thread::spawn(move || {
                let stream = stream.unwrap();
                let reader = BufReader::new(&stream);
                let mut writer = BufWriter::new(&stream);
                for line in reader.lines() {
                    let line = line.unwrap();
                    println!("{}", line);
                    if line.to_string().as_str() == "ping" {
                        writer.write_all(b"pong\n").unwrap();
                        writer.flush().unwrap();
                    }
                }
            });
        }
    }

    #[test]
    pub fn client2() {
        let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
        let mut reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        writer.write_all(b"ping").unwrap();
        writer.flush().unwrap();
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        println!("{}", line);
    }
}
