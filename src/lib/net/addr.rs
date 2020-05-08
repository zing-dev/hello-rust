pub mod socket_addr {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[test]
    fn new() {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        assert_eq!(socket.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(socket.port(), 8080);
        assert_eq!(socket.is_ipv4(), true);
    }

    #[test]
    fn set_ip() {
        let mut addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)), 8080);
        println!("{}", addr.ip().is_ipv4());
        addr.set_ip(IpAddr::from(Ipv4Addr::from([192, 168, 1, 1])));
        println!("{}", addr.ip().to_string());
        addr.set_port(8088);
        println!("{}", addr.port());
        println!("{}", addr.to_string())
    }
}

pub mod socket_addr_v4 {
    use std::net::{Ipv4Addr, SocketAddrV4};

    #[test]
    fn new() {
        let v4 = SocketAddrV4::new(Ipv4Addr::new(192, 168, 0, 111), 8088);
        println!("{}", v4.ip());
        println!("{}", v4.port());
        println!("{}", v4);
    }
}
