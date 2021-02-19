use std::io;
use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

fn main() {}

#[test]
fn test() {
    let mut rdr = Cursor::new(vec![2, 5, 3, 0]);
    // Note that we use type parameters to indicate which kind of byte order
    // we want!
    assert_eq!(517, rdr.read_u16::<BigEndian>().unwrap());
    assert_eq!(768, rdr.read_u16::<BigEndian>().unwrap());
}

#[test]
fn read_u8() {
    let mut rdr = Cursor::new(vec![2, 5]);
    assert_eq!(2, rdr.read_u8().unwrap());
    assert_eq!(5, rdr.read_u8().unwrap());
}

#[test]
fn read_i8() {
    let mut rdr = Cursor::new(vec![0x02, 0xfb]);
    assert_eq!(2, rdr.read_i8().unwrap());
    assert_eq!(-5, rdr.read_i8().unwrap());
}

#[test]
fn read_u16() {
    let mut rdr = Cursor::new(vec![2, 5, 3, 0]);
    assert_eq!(517, rdr.read_u16::<BigEndian>().unwrap());
    assert_eq!(768, rdr.read_u16::<BigEndian>().unwrap());
}

#[test]
fn read_i16() {
    let mut rdr = Cursor::new(vec![0x00, 0xc1, 0xff, 0x7c]);
    assert_eq!(193, rdr.read_i16::<BigEndian>().unwrap());
    assert_eq!(-132, rdr.read_i16::<BigEndian>().unwrap());
}

#[test]
fn read_u24() {
    let mut rdr = Cursor::new(vec![0x00, 0x01, 0x0b]);
    assert_eq!(267, rdr.read_u24::<BigEndian>().unwrap());
}

#[test]
fn write_u8() {
    //let mut wtr = Vec::new();
    let mut wtr = vec![];
    wtr.write_u8(2).unwrap();
    wtr.write_u8(5).unwrap();
    assert_eq!(wtr, b"\x02\x05");
    assert_eq!(wtr, [2, 5]);
}

#[test]
fn write_u32() {
    let mut wtr = vec![];
    wtr.write_u32::<byteorder::BigEndian>(u32::max_value())
        .unwrap();
    assert_eq!(wtr, b"\xff\xff\xff\xff");
    assert_eq!(wtr, [0xff, 0xff, 0xff, 0xff]);
    println!("{:?}", wtr);

    wtr.write_u16::<byteorder::BigEndian>(0xff00).unwrap();
    println!("{:?}", wtr);
    wtr.write_u16::<byteorder::LittleEndian>(0xff00).unwrap();
    println!("{:?}", wtr);
}

impl<W: io::Write + ?Sized> MyWriteBytesExt for W {
    fn hello(&self) {
        println!("hello world")
    }
}

pub trait MyWriteBytesExt: io::Write {
    fn hello(&self);
}

#[test]
fn my() {
    let wtr = Vec::new();
    wtr.hello();
}
