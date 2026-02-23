mod filter;
mod keyboard;
mod device;

use std::time::Duration;
use evdev::Device;
use crate::filter::double_tap::DoubleTapFilter;
use crate::filter::KeyFilter;

fn main() {
    println!("starting program");
    let main_device: Device = device::select_device();
    
    let filters: Vec<Box<dyn KeyFilter>> = vec![
        Box::new(DoubleTapFilter::new((47,48), Duration::from_millis(15)))
    ];
    
    keyboard::run_filter(main_device, filters);
}

