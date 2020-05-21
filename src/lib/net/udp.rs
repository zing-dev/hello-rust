pub mod udp {
    use std::io::Error;
    use std::net::UdpSocket;
    use std::time::Duration;
    use std::{io, thread};

    #[test]
    fn server() -> std::io::Result<()> {
        let socket = UdpSocket::bind("127.0.0.1:34254")?;
        loop {
            let mut buf = [0; 20];
            let (amt, src) = socket.recv_from(&mut buf)?;
            let buf = &mut buf[..amt];
            println!(
                "recv_from from {} => {}",
                src.ip().to_string(),
                String::from_utf8_lossy(buf)
            );
            match socket.send_to(buf, &src) {
                Ok(_) => {}
                Err(_) => {
                    break;
                }
            }
        }
        Ok(())
    }

    #[test]
    fn client() -> std::io::Result<()> {
        let socket = UdpSocket::bind("127.0.0.1:8089")?;
        loop {
            let mut buf = [0; 50];
            let mut str = String::new();
            io::stdin().read_line(&mut str)?;
            match socket.send_to(str.as_bytes(), "127.0.0.1:34254") {
                Ok(_) => {}
                Err(_) => {
                    break;
                }
            }
            let (amt, src) = socket.recv_from(&mut buf)?;
            println!(
                "from {} {} => {}",
                src.ip().to_string(),
                amt,
                String::from_utf8_lossy(&buf)
            );
            thread::sleep(Duration::from_secs(1));
        }
        Ok(())
    }
}
