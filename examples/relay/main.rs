mod client;

use std::ffi::OsString;
use std::io::Write;
use std::time::Duration;
use std::{env, thread};

#[derive(Debug, Clone)]
pub struct Config {
    port: String,
    handle: String,
    branch: Vec<String>,
}

impl Config {
    fn new(args: Vec<String>) -> Config {
        if args.len() < 3 {
            panic!("length")
        }
        let mut branch = Vec::new();
        for arg in 3..args.len() {
            branch.push(args[arg].clone())
        }
        Config {
            port: args[1].clone(),
            handle: args[2].clone(),
            branch,
        }
    }
}

pub fn run(config: Config) {
    let mut port = serial::open(&OsString::from(config.port)).expect("");
    let data: Vec<u8>;
    if config.handle.eq("on") {
        println!("on...");
        data = Vec::from([0x55, 0x01, 0x33, 0xff, 0xff, 0xff, 0xff, 0x85]);
    } else {
        println!("off...");
        data = Vec::from([0x55, 0x01, 0x33, 0x00, 0x00, 0x00, 0x00, 0x89]);
    }
    port.write(&data).unwrap();
    thread::sleep(Duration::from_millis(100))
}

fn run2(config: Config) {
    let mut relay = client::Relay::default(config.clone()).unwrap();
    if config.handle.eq("on") {
        println!("on...{:?}", config.branch);
        for branch in config.branch {
            relay.on(branch.parse::<u8>().unwrap());
        }
    } else if config.handle.eq("off") {
        println!("off... {:?}", config.branch);
        for branch in config.branch {
            relay.off(branch.parse::<u8>().unwrap());
        }
    } else {
        println!("flip... {:?}", config.branch);
        for branch in config.branch {
            relay.flip(branch.parse::<u8>().unwrap());
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    println!("{:?}", args);
    let config = Config::new(args);
    println!("{:?}", config);
    // run(config)
    run2(config);
}
