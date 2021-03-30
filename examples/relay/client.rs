use crate::Config;
use serial::SystemPort;
use std::error::Error;
use std::ffi::OsString;
use std::io::Write;

pub struct Relay {
    address: u8,
    data: [u8; 8],
    port: SystemPort,
}

const DATA_HEADER: u8 = 0x55;
const FUNC_READ_STATUS: u8 = 0x10;
const FUNC_OFF_ONE: u8 = 0x11;
const FUNC_ON_ONE: u8 = 0x12;
const FUNC_FLIP_ONE: u8 = 0x20;

impl Relay {
    pub fn default(config: Config) -> Result<Relay, Box<dyn Error>> {
        Relay::new(1, config)
    }
    pub fn new(address: u8, config: Config) -> Result<Relay, Box<dyn Error>> {
        let port = serial::open(&OsString::from(config.port))?;
        Ok(Relay {
            port,
            address,
            data: [DATA_HEADER, address, 0, 0, 0, 0, 0, 0],
        })
    }
    fn sign(&mut self) {
        let mut sum: u8 = 0;
        for i in 0..7 {
            sum += self.data[i]
        }
        self.data[7] = sum & 0xff;
    }

    fn status(&mut self) {
        self.port.write(&self.data[..]).unwrap();
    }

    pub fn off(&mut self, index: u8) {
        self.data[2] = FUNC_OFF_ONE;
        self.data[3] = 0;
        self.data[4] = 0;
        self.data[5] = 0;
        self.data[6] = index;
        self.sign();
        self.port.write(&self.data[..]).unwrap();
    }

    pub fn on(&mut self, index: u8) {
        self.data[2] = FUNC_ON_ONE;
        self.data[3] = 0;
        self.data[4] = 0;
        self.data[5] = 0;
        self.data[6] = index;
        self.sign();
        self.port.write(&self.data[..]).unwrap();
    }

    pub fn flip(&mut self, index: u8) {
        self.data[2] = FUNC_FLIP_ONE;
        self.data[3] = 0;
        self.data[4] = 0;
        self.data[5] = 0;
        self.data[6] = index;
        self.sign();
        self.port.write(&self.data[..]).unwrap();
    }
}
