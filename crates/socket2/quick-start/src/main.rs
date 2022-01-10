use socket2::{Domain, Socket, Type};
use std::io::{Read, Write};
use std::net::SocketAddr;
use std::{io, thread};

fn main() -> io::Result<()> {
    let mut count = 0;
    let socket = Socket::new(Domain::ipv4(), Type::stream(), None)?;
    socket.bind(&"127.0.0.1:12345".parse::<SocketAddr>().unwrap().into())?;
    socket.listen(128)?;
    let listener = socket.into_tcp_listener();
    for stream in listener.incoming() {
        count += 1;
        thread::spawn(move || -> io::Result<()> {
            let mut stream = stream?;
            let mut data = vec![0; 1024];
            let size = stream.read(&mut data)?;
            println!("---------------------------------------------------");
            println!("{}", String::from_utf8_lossy(&data[..size]));
            let str = format!(
                "HTTP/1.1 200 Ok\r\ncontent-length: {}\r\ncontent-type: text/html\r\n\r\n{}",
                format!("{}", count).len(),
                count
            );
            stream.write(str.as_bytes())?;
            stream.flush()?;
            Ok(())
        });
    }
    Ok(())
}
