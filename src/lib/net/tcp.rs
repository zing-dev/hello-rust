pub mod tcp {
    use chrono::Local;
    use std::io;
    use std::io::{Read, Write};
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn local_addr() {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        assert_eq!(
            listener.local_addr().unwrap(),
            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080))
        );
    }

    fn handle_client(mut stream: TcpStream) {
        let mut data = [0; 256];
        match stream.read(&mut data) {
            Ok(size) => println!("{}:{}", size, String::from_utf8_lossy(&data)),
            Err(err) => println!("{}", err),
        };
    }

    fn run() -> io::Result<()> {
        println!("start");
        let listener = TcpListener::bind("127.0.0.1:8088")?;
        for stream in listener.incoming() {
            println!("one client incoming...");
            handle_client(stream?);
        }
        Ok(())
    }

    #[test]
    fn demo() {
        match run() {
            Ok(_) => println!("OK"),
            Err(err) => println!("err:{}", err),
        }
    }

    #[test]
    fn server() {
        let listener = TcpListener::bind((Ipv4Addr::from([127, 0, 0, 1]), 8088));
        if let Ok(tcp) = listener {
            for result in tcp.incoming() {
                if let Ok(mut stream) = result {
                    let mut data = [0; 64];
                    loop {
                        match stream.read(&mut data) {
                            Ok(size) => {
                                if size == 0 {
                                    continue;
                                }
                                println!("size: {},len: {}", size, data.len());
                                println!("===================================================");
                                println!("{}", String::from_utf8(data.to_vec()).unwrap());
                                println!("===================================================");
                                println!("{}", String::from_utf8_lossy(&data[..size]));
                                println!("===================================================");
                            }
                            Err(err) => {
                                println!("err:{}", err);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn client() {
        let result = TcpStream::connect("127.0.0.1:8088");
        match result {
            Ok(mut stream) => {
                for _ in 1..100 {
                    let mut now = String::from("hello: ");
                    now.push_str(
                        Local::now()
                            .format("%Y-%m-%d %H:%M:%S")
                            .to_string()
                            .as_str(),
                    );
                    let result = stream.write(now.as_bytes());
                    match result {
                        Ok(size) => {
                            println!("{}", size);
                        }
                        Err(err) => println!("{}", err),
                    }
                    let result = stream.flush();
                    match result {
                        Ok(_) => {
                            sleep(Duration::from_secs(3));
                        }
                        Err(err) => println!("{}", err),
                    }
                }
            }
            Err(err) => println!("{}", err),
        }
    }
}
