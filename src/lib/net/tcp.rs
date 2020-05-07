pub mod tcp {

    use std::io;
    use std::io::Read;
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};

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
}
