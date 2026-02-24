mod filter;
mod keyboard;
mod device;
mod ui;


use crate::filter::KeyFilter;

fn main() -> iced::Result {
    println!("starting program");
    iced::run(ui::app::App::update, ui::app::App::view)

    /*
    let main_device: Device = device::select_device();
    
    let filters: Vec<Box<dyn KeyFilter>> = vec![
        Box::new(DoubleTapFilter::new((47,48), Duration::from_millis(15)))
    ];

    keyboard::run_filter(main_device, filters);

     */
    
}

