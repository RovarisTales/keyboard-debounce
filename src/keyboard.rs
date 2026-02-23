use evdev::{Device, EventType, InputEvent, KeyCode};
use evdev::EventSummary::Key;
use evdev::uinput::VirtualDevice;
use crate::device;
use crate::filter::{FilterResult, KeyFilter};

pub fn run_filter(mut main_device: Device, mut filters: Vec<Box<dyn KeyFilter>> /* a vector of pointers to implementations of KeyFilter */) {
    let mut virtual_keyboard:VirtualDevice = device::create_virtual_keyboard(&mut main_device);
    
    device::flush_device(&mut main_device);

    'main: loop {
        for  event in main_device.fetch_events().unwrap() {
            match event.destructure() {
                Key(_, KeyCode::KEY_9, 1) => {
                    println!("shutting down program");
                    break 'main;
                },

                Key(_, key, event_code) => {
                    let result = filters.iter_mut().fold(
                        FilterResult::Pass,
                        |accumulator,filter| {
                            match accumulator {
                                FilterResult::Suppress => FilterResult::Suppress,
                                FilterResult::Pass => filter.filter(key, event_code),
                            }
                        }
                    );
                    match result {
                        FilterResult::Pass => {
                            let input = InputEvent::new(EventType::KEY.0, key.code(), event_code);
                            virtual_keyboard.emit(&[input]).expect("Virtual keyboard emit failed");
                        },
                        FilterResult::Suppress => {
                            println!("Suppressed for key {}", key.code());
                        }
                    }
                }


                _ => {}
            }
        }
    }
    main_device.ungrab().expect("UNGRAB FAILED RESTART PC");
}
