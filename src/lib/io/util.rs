use std::io;
use std::io::{Read, Write};

#[test]
fn copy() {
    let mut reader: &[u8] = b"hello";
    let mut writer: Vec<u8> = vec![];
    println!("{:?}", &reader.len());
    println!("{:?}", &reader[..]);
    match io::copy(&mut reader, &mut writer) {
        Ok(_) => {}
        Err(_) => {}
    }
    assert_eq!(&b"hello"[..], &writer[..]);
    println!("{:?}", writer);
    println!("{:?}", String::from_utf8(writer).unwrap());
    println!("{:?}", &reader.len());
    println!("{:?}", &reader[..]);
}

#[test]
fn empty() {
    let mut buffer = String::new();
    io::empty().read_to_string(&mut buffer).unwrap();
    assert!(buffer.is_empty());
}

#[test]
fn repeat() {
    let mut buffer = [0; 3];
    io::repeat(0b101).read_exact(&mut buffer).unwrap();
    assert_eq!(buffer, [0b101, 0b101, 0b101]);
}

#[test]
fn sink() {
    let buffer = vec![1, 2, 3, 5, 8];
    let num_bytes = io::sink().write(&buffer).unwrap();
    assert_eq!(num_bytes, 5);
}
