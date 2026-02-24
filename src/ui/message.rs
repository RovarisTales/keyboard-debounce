use std::time::Duration;

#[derive(Debug, Clone)]
pub enum Message {
    KeySelected(u16),
    KeyDeselected(u16),
    Debounce(Duration),
    StartFilter,
    StopFilter,
    SelectDevice(usize),
    RemoveDevice
}