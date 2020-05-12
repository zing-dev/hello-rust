extern crate serial;

use serial::prelude::*;
use std::time::Duration;
use std::{env, io};

fn main() -> io::Result<()> {
    let mut port = if env::args_os().len() > 1 {
        serial::open(env::args_os().next().unwrap().as_os_str())?
    } else {
        serial::open("COM21")?
    };
    interact(&mut port)?;
    Ok(())
}

#[allow(unreachable_code)]
fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud9600)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })?;
    port.set_timeout(Duration::from_millis(1000))?;
    let mut buf: Vec<u8> = (0..255).collect();
    //println!("{:?}", buf);
    port.write(&buf[..])?;
    loop {
        match port.read(&mut buf[..]) {
            Ok(size) => {
                println!("{}", String::from_utf8_lossy(&buf[0..size]));
                // println!("{:?}", &buf[0..size]);
            }
            Err(_) => {}
        };
    }
    Ok(())
}
