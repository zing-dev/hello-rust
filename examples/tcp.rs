use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut data = [0; 256];
    match stream.read(&mut data) {
        Ok(size) => {
            println!("{}:{}", size, String::from_utf8_lossy(&data));
            stream.write(&data);
        }
        Err(err) => println!("{}", err),
    };
}
fn main() -> io::Result<()> {
    println!("start");
    let listener = TcpListener::bind("127.0.0.1:8088")?;
    for stream in listener.incoming() {
        println!("one client incoming...");
        handle_client(stream?);
    }
    Ok(())
}
