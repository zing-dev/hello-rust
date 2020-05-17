use socket2::{Domain, Socket, Type};
use std::io::{Read, Write};
use std::net::SocketAddr;
use std::{io, thread};

fn main() -> io::Result<()> {
    let socket = Socket::new(Domain::ipv4(), Type::stream(), None)?;
    socket.bind(&"127.0.0.1:12345".parse::<SocketAddr>().unwrap().into())?;
    socket.listen(128)?;
    let listener = socket.into_tcp_listener();
    for stream in listener.incoming() {
        thread::spawn(|| -> io::Result<()> {
            let mut stream = stream?;
            let mut data = vec![0; 255];
            let size = stream.read(&mut data)?;
            println!("{}", String::from_utf8_lossy(&data[..size]));
            stream.write(
                "HTTP/1.1 200 Ok\r\ncontent-length: 1\r\ncontent-type: text/html\r\n\r\n1"
                    .as_bytes(),
            )?;
            stream.flush()?;
            Ok(())
        });
    }
    Ok(())
}
