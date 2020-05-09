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

    mod single_threaded {
        use std::fs;
        use std::io::prelude::*;
        use std::net::TcpListener;
        use std::net::TcpStream;

        #[test]
        fn main() {
            let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

            for stream in listener.incoming() {
                let stream = stream.unwrap();
                println!("===================new client===================");
                handle_connection4(stream);
            }
        }

        fn handle_connection(mut stream: TcpStream) {
            let mut buffer = [0; 512];
            stream.read(&mut buffer).unwrap();
            println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        }

        fn handle_connection2(mut stream: TcpStream) {
            let mut buffer = [0; 512];
            stream.read(&mut buffer).unwrap();
            println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
            let response = "HTTP/1.1 200 OK\r\n\r\n";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }

        fn handle_connection3(mut stream: TcpStream) {
            let mut buffer = [0; 512];
            stream.read(&mut buffer).unwrap();
            let contents = fs::read_to_string("src/lib/net/hello.html").unwrap();
            let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }

        fn handle_connection4(mut stream: TcpStream) {
            let mut buffer = [0; 512];
            stream.read(&mut buffer).unwrap();
            let get = b"GET / HTTP/1.1\r\n";
            if buffer.starts_with(get) {
                let contents = fs::read_to_string("src/lib/net/hello.html").unwrap();
                let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            } else {
                let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
                let contents = fs::read_to_string("src/lib/net/404.html").unwrap();
                let response = format!("{}{}", status_line, contents);
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        }

        fn handle_connection5(mut stream: TcpStream) {
            let mut buffer = [0; 512];
            stream.read(&mut buffer).unwrap();
            let get = b"GET / HTTP/1.1\r\n";
            let (status_line, filename) = if buffer.starts_with(get) {
                ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
            } else {
                ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
            };

            let contents = fs::read_to_string("src/lib/net/".to_owned() + filename).unwrap();
            let response = format!("{}{}", status_line, contents);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }

    mod multi_threaded {
        use std::io::{Read, Write};
        use std::net::{TcpListener, TcpStream};
        use std::time::Duration;
        use std::{fs, thread};

        #[test]
        fn main() {
            let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                println!("===================new client===================");
                handle_connection(stream);
            }
        }

        #[test]
        fn main2() {
            let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
        }

        fn handle_connection(mut stream: TcpStream) {
            let mut buffer = [0; 512];
            stream.read(&mut buffer).unwrap();
            let get = b"GET / HTTP/1.1\r\n";
            let sleep = b"GET /sleep HTTP/1.1\r\n";

            let (status_line, filename) = if buffer.starts_with(get) {
                ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
            } else if buffer.starts_with(sleep) {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
            } else {
                ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
            };
            let contents = fs::read_to_string("src/lib/net/".to_owned() + filename).unwrap();
            let response = format!("{}{}", status_line, contents);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
