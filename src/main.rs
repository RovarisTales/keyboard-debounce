use std::collections::HashMap;
use std::time::{Duration, Instant};
use evdev::{Device, EventType, InputEvent, KeyCode};
use evdev::EventSummary::Key;
use evdev::uinput::{VirtualDevice};

fn main() {
    println!("starting program");
    let mut main_device: Device;
    for (_path, device) in evdev::enumerate() {
        if device.name().unwrap().contains("Wooting") {

            if device.physical_path().unwrap().contains("input1") {

                //println!("{device}");
                main_device = device;
                initialize_debounce_timer(main_device);
            }

        }
    }

}
/*

    need to start timer when key pressed
    when 1 ms timeout allow key again
    if key pressed again when timer is running ignore input
    how to check if key is active?
    global map? (maybe better for a set) that when press is active it is in map
    after timeout remove from map
    handle concurrency if removing and adding happens at the same time


 */
fn initialize_debounce_timer(mut main_device: Device) {
    let mut virtual_keyboard;
    match main_device.grab()  {
        Err(err) => panic!("grab error: {}", err),
        Ok(_) => {
            virtual_keyboard = VirtualDevice::builder().unwrap()
                .name("Virtual Wooting")
                .input_id(main_device.input_id())
                .with_keys(main_device.supported_keys().unwrap())
                .unwrap().build().unwrap();
        }
    };
    //  code 1 is for press , code 0a is for release

    /*
        Scenarios:
            1 -> key presses 2 times without releasing (bad) : ignore second input
            2 -> key released but pressed again in less than 1 ms (bad too) : ignore input
        fn handle_key_press needs to check both timers for pressing again and releasing
        if any of those are still active ignore input
        set 1 for press timer
        set 2 for release timer
        when pressing add to press timer
        when releasing remove from press timer and add to release timer
        no need for threads, events from keyboard are sequential on evdev, parallelism adds complexity and very to little performance gain
*/
    let mut press_map: HashMap<u16, Instant> = HashMap::new();



    const KEYCODE_1: u16 = 47;
    const KEYCODE_2: u16 = 48;
    'main: loop {
        for  event in main_device.fetch_events().unwrap() {
            match event.destructure() {
                Key(_, KeyCode::KEY_9, 1) => {
                    println!("shutting down program");
                    break 'main;
                },
                //press
                Key(_, key, 1) => {
                    let now = Instant::now();
                    let should_supress = {
                        let key_to_check = match key.code() {
                            KEYCODE_1 => Some(KEYCODE_2),
                            KEYCODE_2 => Some(KEYCODE_1),
                            _ => None,
                        };
                        key_to_check
                            .and_then( |key_code| press_map.get(&key_code))
                            .is_some_and(|inst| now - *inst < Duration::from_millis(15))
                    };

                    if should_supress {
                        println!("Double tap recognized for key {}, ignoring", key.code())
                    } else {
                        let input = InputEvent::new(EventType::KEY.0, key.code(), 1);
                        virtual_keyboard.emit(&[input]).expect("Virtual keyboard emit failed");
                    }

                    press_map.insert(key.code(), now);

                },
                //release and repeat
                Key(_, key, val) => {
                    let input = InputEvent::new(EventType::KEY.0, key.code(), val);
                    virtual_keyboard.emit(&[input]).expect("Virtual keyboard emit failed");
                },


                _ => {}
            }
        }
    }
    main_device.ungrab().expect("UNGRAB FAILED RESTART PC");
}