/*
    Create the ui for the keyboard filtering allowing users to select 2 keys for filtering, for osu!
    initial ui has these fields
    filter choice then keyboard ui then debounce time and then create the filter
    keyboard layout to select keys to filter
    list of available filters
    delay time for filtering
    start and stop button
*/
use evdev::{Device, KeyCode};
use iced::{Element, Fill};
use crate::filter::KeyFilter;
use crate::ui::message::Message;
use iced::widget::{button, column, container, row, scrollable, text, Column, Row};

pub struct App {
    filters: Vec<Box<dyn KeyFilter>>,
    devices: Vec<Device>,
    selected_device: Option<Device>,
}

impl Default for App {
    fn default() -> Self {
        let devices: Vec<evdev::Device> = evdev::enumerate()
            .map(|(_, device)| device)
            .collect();
        App { filters: Vec::new(), devices, selected_device: None }
    }
}

impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SelectDevice(device_index) => {
                self.selected_device = Some(self.devices.remove(device_index)); //TODO add device back to list when removed and check remove impl to be warry of bugs
            }
            _ => {}
        }
    }

    pub fn view(app: &App) -> Element<'_, Message> {

        let content: Element<'_, Message> = match &app.selected_device {
            None => {
                let mut col: Column<'_, Message> = column![
                    text("Select a device"),
                ];
                for (i, device) in app.devices.iter().enumerate() {
                    col = col.push(
                        button(text(device.name().unwrap())).on_press(Message::SelectDevice(i))
                    );
                }
                scrollable(col.spacing(10)).into()
            }
            Some(device) => {

                let keys: Vec<KeyCode> = device.supported_keys()
                    .unwrap()
                    .iter()
                    .collect();
                for (i, key) in keys.iter().enumerate() {
                    println!("{}", device)
                }
                let chunk_size = (keys.len() + 3) / 4; // round up division
                let mut col = column![text("Select keys to filter:")];
                for chunk in keys.chunks(chunk_size) {
                    let mut row:Row<'_, Message> = row![].spacing(3);
                    for (i, key) in chunk.iter().enumerate() {
                        row = row.push(button(text(key.code())).on_press(Message::KeySelected(key.code())));
                    }
                    col = col.push(row);
                }
                scrollable(col).into()
            }
        };

        container(content)
            .padding(10)
            .center(Fill)
            .into()
    }

}