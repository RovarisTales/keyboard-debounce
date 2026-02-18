use evdev::{Device, EventSummary, KeyCode};

fn main() {
    println!("starting program");
    let mut main_device: Device;
    for (path, device) in evdev::enumerate() {
        if device.name().unwrap().contains("Wooting") {

            if device.physical_path().unwrap().contains("input1") {

                println!("{device}");
                main_device = device;
                initialize_debounce_timer(main_device);
            }

        }
    }






}

fn initialize_debounce_timer(mut main_device: Device) {
    //    main_device.grab();
    //  code 1 is for press , code 0a is for release
    loop {
        for  event in main_device.fetch_events().unwrap() {
           match event.destructure() {
              EventSummary::Key(ev, KeyCode::KEY_A, 1) => {

                  println!("{ev:?}");

              },

               // 3. Handle Key Release (Value 0)
              EventSummary::Key(_, KeyCode::KEY_A, 0) => {
                  println!("Released A");
              },


              _ => {}
           }
        }
    }
}