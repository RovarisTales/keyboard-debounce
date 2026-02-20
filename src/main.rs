use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use dashmap::DashSet;
use evdev::{Device, EventSummary};
use web_server::ThreadPool;

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
    //    main_device.grab();
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


    let press_set: Arc<DashSet<u16>> = Arc::new(DashSet::new());
    let release_set: Arc<DashSet<u16>> = Arc::new(DashSet::new());
    let thread_pool: ThreadPool = ThreadPool::new(8);
    let mut key_hash_map_press: HashMap<u16,Instant> = HashMap::new();
    let mut key_hash_map_release: HashMap<u16,Instant> = HashMap::new();

     */
    let mut press_map: HashMap<u16, Vec<Instant>> = HashMap::new();
    let mut release_map: HashMap<u16, Vec<Instant>> = HashMap::new();


    loop {
        for  event in main_device.fetch_events().unwrap() {
            match event.destructure() {
                EventSummary::Key(_, key, 1) if key == evdev::KeyCode::KEY_1 => {
                    println!("press map for key 47 {:?}", press_map.entry(47));
                    println!("release map for key 47 {:?}", release_map.entry(47));
                    println!("press map for key 47 {:?}", press_map.entry(48));
                    println!("release map for key 47 {:?}", release_map.entry(48));

                },
                EventSummary::Key(_, key, 1) => {
                    /*
                    let key_code = key.code();
                    let press_c = Arc::clone(&press_set);
                    let release_c = Arc::clone(&release_set);
                    thread_pool.execute(move || {
                        handle_key_press(key_code, press_c, release_c);
                    });

                     */
                    press_map.entry(key.code())
                        .or_insert_with(Vec::new)
                        .push(Instant::now());
                },

                EventSummary::Key(_, key, 0) => {
                    /*
                    let key_code = key.code();
                    let press_c = Arc::clone(&press_set);
                    let release_c = Arc::clone(&release_set);
                    thread_pool.execute(move || {
                        handle_key_release(key_code, press_c, release_c);
                    });

                     */
                    release_map.entry(key.code())
                        .or_insert_with(Vec::new)
                        .push(Instant::now());

                },


                _ => {}
            }
        }
    }
}

fn handle_key_press(key: u16, press_set: Arc<DashSet<u16>>, release_set: Arc<DashSet<u16>>) {
    if release_set.contains(&key) {
        // case where key was released but timer is not over yet
        println!("ALERT OF DOUBLE TAP RELEASE HANDLE THIS CASE FOR THIS KEY: {}", key);
        return;
    }
    if press_set.contains(&key) {
        // case where key is pressed and new event comes in
        println!("ALERT OF DOUBLE TAP PRESS HANDLE THIS CASE FOR THIS KEY: {}", key);
        return;
    }
    press_set.insert(key);
    // add none of those were true, press key and allow input
    // println!("happy path");
}

/// Handles release events to the virtual keyboard
///
/// sends release event to virtual keyboard
fn handle_key_release(key: u16, press_set: Arc<DashSet<u16>>, release_set: Arc<DashSet<u16>>) {
    // key NEEDS to be in press set or else something wrong is going on
    // open statement is wrong, key is removed from pressed set as the first action and then can be released without having beeing pressed

    press_set.remove(&key);
    release_set.insert(key);
    thread::sleep(Duration::from_millis(1000));
    release_set.remove(&key);
    //println!("key released successfully");

}
// press
// release
// press (goes into return clause)
// release -> panic