pub fn addr() {
    use std::net::*;

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    assert_eq!("127.0.0.1:8080".parse(), Ok(socket));
    assert_eq!(socket.port(), 8080);
    assert_eq!(socket.is_ipv4(), true);
    assert_eq!(socket.is_ipv6(), false);

    let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    assert_eq!("127.0.0.1:8080".parse(), Ok(socket));
    assert_eq!(socket.ip(), &Ipv4Addr::new(127, 0, 0, 1));
    assert_eq!(socket.port(), 8080);

    let socket = SocketAddrV6::new(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1), 8080, 0, 0);
    assert_eq!("[2001:db8::1]:8080".parse(), Ok(socket));
    assert_eq!(socket.ip(), &Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1));
    assert_eq!(socket.port(), 8080);

    //set_ip
    let mut socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    socket.set_ip(IpAddr::V4(Ipv4Addr::new(10, 10, 0, 1)));
    assert_eq!(socket.ip(), IpAddr::V4(Ipv4Addr::new(10, 10, 0, 1)));

    //port
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    assert_eq!(socket.port(), 8080);

    //set_port
    let mut socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    socket.set_port(1025);
    assert_eq!(socket.port(), 1025);

    //flowinfo
    let socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 10, 0);
    assert_eq!(socket.flowinfo(), 10);

    //set_flowinfo
    let mut socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 10, 0);
    socket.set_flowinfo(56);
    assert_eq!(socket.flowinfo(), 56);

    //scope_id
    let socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 78);
    assert_eq!(socket.scope_id(), 78);

    //set_scope_id
    let mut socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 78);
    socket.set_scope_id(42);
    assert_eq!(socket.scope_id(), 42);

    let addr = SocketAddr::from(([127, 0, 0, 1], 443));
    let mut addrs_iter = addr.to_socket_addrs().unwrap();
    assert_eq!(Some(addr), addrs_iter.next());
    assert!(addrs_iter.next().is_none());

//    let mut addrs_iter = "localhost:443".to_socket_addrs().unwrap();
//    assert_eq!(addrs_iter.next(), Some(SocketAddr::from(([127, 0, 0, 1], 443))));
//    assert!(addrs_iter.next().is_none());
//     assuming 'foo' does not resolve
//    assert!("foo:443".to_socket_addrs().is_err());

    let addr1 = SocketAddr::from(([0, 0, 0, 0], 80));
    let addr2 = SocketAddr::from(([127, 0, 0, 1], 443));
    let addrs = vec![addr1, addr2];
    let mut addrs_iter = (&addrs[..]).to_socket_addrs().unwrap();
    assert_eq!(Some(addr1), addrs_iter.next());
    assert_eq!(Some(addr2), addrs_iter.next());
    assert!(addrs_iter.next().is_none());

//    use std::io;
//    let err = "127.0.0.1".to_socket_addrs().unwrap_err();
//    assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
}

pub fn ip() {
    use std::net::*;
    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    assert_eq!("127.0.0.1".parse(), Ok(localhost_v4));
    assert_eq!("::1".parse(), Ok(localhost_v6));
    assert_eq!(localhost_v4.is_ipv6(), false);
    assert_eq!(localhost_v4.is_ipv4(), true);

    let localhost = Ipv4Addr::new(127, 0, 0, 1);
    assert_eq!("127.0.0.1".parse(), Ok(localhost));
    assert_eq!(localhost.is_loopback(), true);

    let localhost = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
    assert_eq!("::1".parse(), Ok(localhost));
    assert_eq!(localhost.is_loopback(), true);

    //is_unspecified
    assert_eq!(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)).is_unspecified(), true);
    assert_eq!(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)).is_unspecified(), true);

    //is_loopback
    assert_eq!(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)).is_loopback(), true);
    assert_eq!(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0x1)).is_loopback(), true);

    //is_global
//    assert_eq!(IpAddr::V4(Ipv4Addr::new(80, 9, 12, 3)).is_global(), true);
//    assert_eq!(IpAddr::V6(Ipv6Addr::new(0, 0, 0x1c9, 0, 0, 0xafc8, 0, 0x1)).is_global(), true);

    //is_multicast
    assert_eq!(IpAddr::V4(Ipv4Addr::new(224, 254, 0, 0)).is_multicast(), true);
    assert_eq!(IpAddr::V6(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0)).is_multicast(), true);

    //is_documentation
//    assert_eq!(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 6)).is_documentation(), true);
//    assert_eq!(IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0)) .is_documentation(), true);
//
    //new
    let _addr = Ipv4Addr::new(127, 0, 0, 1);
    let addr = Ipv4Addr::LOCALHOST;
    assert_eq!(addr, Ipv4Addr::new(127, 0, 0, 1));
    let addr = Ipv4Addr::UNSPECIFIED;
    assert_eq!(addr, Ipv4Addr::new(0, 0, 0, 0));
    let addr = Ipv4Addr::BROADCAST;
    assert_eq!(addr, Ipv4Addr::new(255, 255, 255, 255));

    //octets
    let addr = Ipv4Addr::new(127, 0, 0, 1);
    assert_eq!(addr.octets(), [127, 0, 0, 1]);

    //is_unspecified
    assert_eq!(Ipv4Addr::new(0, 0, 0, 0).is_unspecified(), true);
    assert_eq!(Ipv4Addr::new(45, 22, 13, 197).is_unspecified(), false);

    //is_private
    assert_eq!(Ipv4Addr::new(10, 0, 0, 1).is_private(), true);
    assert_eq!(Ipv4Addr::new(10, 10, 10, 10).is_private(), true);
    assert_eq!(Ipv4Addr::new(172, 16, 10, 10).is_private(), true);
    assert_eq!(Ipv4Addr::new(172, 29, 45, 14).is_private(), true);
    assert_eq!(Ipv4Addr::new(172, 32, 0, 2).is_private(), false);
    assert_eq!(Ipv4Addr::new(192, 168, 0, 2).is_private(), true);
    assert_eq!(Ipv4Addr::new(192, 169, 0, 2).is_private(), false);

    //is_link_local
    assert_eq!(Ipv4Addr::new(169, 254, 0, 0).is_link_local(), true);
    assert_eq!(Ipv4Addr::new(169, 254, 10, 65).is_link_local(), true);
    assert_eq!(Ipv4Addr::new(16, 89, 10, 65).is_link_local(), false);

    //is_shared
//    assert_eq!(Ipv4Addr::new(100, 64, 0, 0).is_shared(), true);
//    assert_eq!(Ipv4Addr::new(100, 127, 255, 255).is_shared(), true);
//    assert_eq!(Ipv4Addr::new(100, 128, 0, 0).is_shared(), false);

    //is_multicast
    assert_eq!(Ipv4Addr::new(224, 254, 0, 0).is_multicast(), true);
    assert_eq!(Ipv4Addr::new(236, 168, 10, 65).is_multicast(), true);
    assert_eq!(Ipv4Addr::new(172, 16, 10, 65).is_multicast(), false);

    //is_broadcast
    assert_eq!(Ipv4Addr::new(255, 255, 255, 255).is_broadcast(), true);
    assert_eq!(Ipv4Addr::new(236, 168, 10, 65).is_broadcast(), false);

    //to_ipv6_compatible
    assert_eq!(Ipv4Addr::new(192, 0, 2, 255).to_ipv6_compatible(),
               Ipv6Addr::new(0, 0, 0, 0, 0, 0, 49152, 767));

    //to_ipv6_mapped
    assert_eq!(Ipv4Addr::new(192, 0, 2, 255).to_ipv6_mapped(),
               Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 49152, 767));

    let addr = Ipv4Addr::new(13, 12, 11, 10);
    assert_eq!(0x0d0c0b0au32, u32::from(addr));
    let addr = Ipv4Addr::from(0x0d0c0b0au32);
    assert_eq!(Ipv4Addr::new(13, 12, 11, 10), addr);

    let addr = Ipv4Addr::from([13u8, 12u8, 11u8, 10u8]);
    assert_eq!(Ipv4Addr::new(13, 12, 11, 10), addr);

    let addr = IpAddr::from([13u8, 12u8, 11u8, 10u8]);
    assert_eq!(IpAddr::V4(Ipv4Addr::new(13, 12, 11, 10)), addr);
}

#[allow(unused_variables)]
#[allow(unused_must_use)]
pub fn tcp() {
    use std::io::prelude::*;
    use std::net::*;

    fn main() -> std::io::Result<()> {
        let mut stream = TcpStream::connect("127.0.0.1:8080")?;
        stream.write(&[72])?;
        stream.read(&mut [0; 128])?;
        Ok(())
    }
    main();

    if let Ok(stream) = TcpStream::connect("127.0.0.1:8080") {
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server...");
    }

    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 8080)),
        SocketAddr::from(([127, 0, 0, 1], 8081)),
    ];
    if let Ok(mut stream) = TcpStream::connect(&addrs[..]) {
        println!("Connected to the server!");
        stream.write(&[70]);
    } else {
        println!("Couldn't connect to server...");
    }

    let stream = TcpStream::connect("127.0.0.1:8080")
        .expect("Couldn't connect to the server...");
    assert_eq!(stream.peer_addr().unwrap(),
               SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080)));

    let stream = TcpStream::connect("127.0.0.1:8080")
        .expect("Couldn't connect to the server...");
    assert_eq!(stream.local_addr().unwrap().ip(),
               IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

    let stream = TcpStream::connect("127.0.0.1:8080")
        .expect("Couldn't connect to the server...");
    stream.shutdown(Shutdown::Both).expect("shutdown call failed");

    let stream = TcpStream::connect("127.0.0.1:8080")
        .expect("Couldn't connect to the server...");
    let _stream_clone = stream.try_clone().expect("clone failed...");
}
