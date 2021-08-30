pub mod list_devices;

#[test]
fn version() {
    let v = libusb::version();
    println!("libusb v{}.{}.{}.{}`{}", v.major(), v.minor(), v.micro(), v.nano(), v.rc().unwrap_or(""));

    let mut context = match libusb::Context::new() {
        Ok(c) => c,
        Err(e) => panic!("libusb::Context::new(): {}", e)
    };

    context.set_log_level(libusb::LogLevel::Debug);
    context.set_log_level(libusb::LogLevel::Info);
    context.set_log_level(libusb::LogLevel::Warning);
    context.set_log_level(libusb::LogLevel::Error);
    context.set_log_level(libusb::LogLevel::None);

    println!("has capability? {}", context.has_capability());
    println!("has hotplug? {}", context.has_hotplug());
    println!("has HID access? {}", context.has_hid_access());
    println!("supports detach kernel driver? {}", context.supports_detach_kernel_driver())
}

fn main() {
    let context = libusb::Context::new().unwrap();
    for device in context.devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        println!("Bus bus_number{:03} Device address {:03} ID vendor_id {:04x}:product_id {:04x}",
                 device.bus_number(),
                 device.address(),
                 device_desc.vendor_id(),
                 device_desc.product_id());
    }

    println!("has_capability {}", context.has_capability());
    println!("has_hid_access {}", context.has_hid_access());
    println!("has_hotplug {}", context.has_hotplug());
}
