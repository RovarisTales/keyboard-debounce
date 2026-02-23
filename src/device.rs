use std::io::stdin;
use evdev::Device;
use evdev::uinput::VirtualDevice;

pub fn select_device() -> Device {
    let mut devices: Vec<Device> = evdev::enumerate()
        .map(|(_, device)| device)
        .collect();

    loop {
        println!("Select a device for doubletap filtering:");
        for (i, device) in devices.iter().enumerate() {
            println!("  {}: {}", i, device.name().unwrap_or("Unknown"));
        }

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");

        let index: usize = match input.trim().parse() {
            Ok(n) if n < devices.len() => n,
            _ => {
                println!("Invalid selection, try again.");
                continue;
            }
        };

        return devices.swap_remove(index);
    }
}

pub fn flush_device(device:&mut Device) {
    device.set_nonblocking(true).unwrap();

    // Flush pending events
    while let Ok(events) = device.fetch_events() {
        let _ = events.count();
    }

    // Back to blocking for the main loop
    device.set_nonblocking(false).unwrap();
}

pub fn create_virtual_keyboard(main_device:&mut Device) -> VirtualDevice{
    match main_device.grab()  {
        Err(err) => panic!("grab error: {}", err),
        Ok(_) => {
            VirtualDevice::builder().unwrap()
                .name("Virtual Wooting")
                .input_id(main_device.input_id())
                .with_keys(main_device.supported_keys().unwrap())
                .unwrap().build().unwrap()
        }
    }
}