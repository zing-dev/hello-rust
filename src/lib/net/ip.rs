pub mod ip_addr {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    #[test]
    //未指定
    fn is_unspecified() {
        assert_eq!(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)).is_unspecified(), true);
        assert_eq!(
            IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)).is_unspecified(),
            true
        );
    }

    #[test]
    //回环地址
    fn is_loopback() {
        assert_eq!(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)).is_loopback(), true);
        assert_eq!(
            IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0x1)).is_loopback(),
            true
        );
    }

    //error[E0658]: use of unstable library feature 'ip': extra functionality has not been scrutinized to the level that it should be to be stable
    fn is_global() {
        /* assert_eq!(IpAddr::V4(Ipv4Addr::new(80, 9, 12, 3)).is_global(), true);
        assert_eq!(
            IpAddr::V6(Ipv6Addr::new(0, 0, 0x1c9, 0, 0, 0xafc8, 0, 0x1)).is_global(),
            true
        );*/
    }

    #[test]
    //多播地址
    fn is_multicast() {
        assert_eq!(
            IpAddr::V4(Ipv4Addr::new(224, 254, 0, 0)).is_multicast(),
            true
        );
        assert_eq!(
            IpAddr::V6(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0)).is_multicast(),
            true
        );
    }
}

pub mod ipv4_addr {
    use std::net::Ipv4Addr;

    #[test]
    fn new() {
        let addr = Ipv4Addr::new(127, 0, 0, 0);
        println!("{:?}", addr);
        println!("{:?}", Ipv4Addr::new(192, 168, 0, 1));
        println!("{:?}", Ipv4Addr::LOCALHOST); //127.0.0.1
        println!("{:?}", Ipv4Addr::UNSPECIFIED); //0.0.0.0
        println!("{:?}", Ipv4Addr::BROADCAST); //255.255.255.255
    }

    #[test]
    fn octets() {
        println!("{:?}", Ipv4Addr::LOCALHOST.octets()); //[127, 0, 0, 1]
    }

    #[test]
    fn is_unspecified() {
        println!("{:?}", Ipv4Addr::LOCALHOST.is_unspecified()); //[127, 0, 0, 1]
    }
}
