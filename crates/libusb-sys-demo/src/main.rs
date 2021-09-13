use std::{slice};
use std::mem::MaybeUninit;
use std::os::raw::c_int;

use libusb_sys::{libusb_context, libusb_device, libusb_exit, libusb_free_device_list, libusb_get_bus_number, libusb_get_device_address, libusb_get_device_list, libusb_get_device_speed, libusb_get_parent, libusb_get_port_number, libusb_get_port_numbers, libusb_init};

fn main() {
    let mut context: *mut libusb_context = unsafe { MaybeUninit::uninit().assume_init() };

    match unsafe { libusb_init(&mut context) } {
        0 => println!("init ok"),
        e => panic!("libusb_init: {}", get_error(e))
    };

    list_devices(context);

    unsafe { libusb_exit(context) };
}


fn list_devices(context: *mut libusb_context) {
    let mut device_list: *const *mut libusb_device = unsafe { MaybeUninit::uninit().assume_init() };

    let len = unsafe { libusb_get_device_list(context, &mut device_list) };

    if len < 0 {
        println!("libusb_get_device_list: {}", get_error(len as c_int));
        return;
    }
    println!("libusb_get_device_list: {}", len);

    let devs = unsafe { slice::from_raw_parts(device_list, len as usize) };
    for dev in devs {
        display_device(dev);
    }

    unsafe { libusb_free_device_list(device_list, 1) };
}

fn display_device(dev: &*mut libusb_device) {
    let bus = unsafe { libusb_get_bus_number(*dev) };
    let address = unsafe { libusb_get_device_address(*dev) };
    let port = unsafe { libusb_get_port_number(*dev) };
    let speed = unsafe { libusb_get_device_speed(*dev) };
    println!("bus:{},port:{},address:{},speed:{}", bus, port, address, speed);

    unsafe {
        let p = libusb_get_parent(*dev);
        if p.is_null() {
            println!("no parent");
        } else {
            let port_number = libusb_get_port_number(p);
            let bus = libusb_get_bus_number(p);
            println!("parent bus:{}, port:{}", bus, port_number)
        }
    }
    let mut ports: [u8; 8] = [0; 8];
    let res = unsafe { libusb_get_port_numbers(*dev, &mut ports as *mut u8, ports.len() as c_int) };
    if res > 0 {
        print!("path: {}", ports[0]);
        let mut i: u8 = 1;
        while i < res as u8 {
            print!(".{}", ports[i as usize]);
            i += 1;
        }
    }
    println!();
}

fn get_error(err: c_int) -> &'static str {
    match err {
        libusb_sys::LIBUSB_SUCCESS => "success",
        libusb_sys::LIBUSB_ERROR_IO => "I/O error",
        libusb_sys::LIBUSB_ERROR_INVALID_PARAM => "invalid parameter",
        libusb_sys::LIBUSB_ERROR_ACCESS => "access denied",
        libusb_sys::LIBUSB_ERROR_NO_DEVICE => "no such device",
        libusb_sys::LIBUSB_ERROR_NOT_FOUND => "entity not found",
        libusb_sys::LIBUSB_ERROR_BUSY => "resource busy",
        libusb_sys::LIBUSB_ERROR_TIMEOUT => "opteration timed out",
        libusb_sys::LIBUSB_ERROR_OVERFLOW => "overflow error",
        libusb_sys::LIBUSB_ERROR_PIPE => "pipe error",
        libusb_sys::LIBUSB_ERROR_INTERRUPTED => "system call interrupted",
        libusb_sys::LIBUSB_ERROR_NO_MEM => "insufficient memory",
        libusb_sys::LIBUSB_ERROR_NOT_SUPPORTED => "operation not supported",
        libusb_sys::LIBUSB_ERROR_OTHER | _ => "other error"
    }
}