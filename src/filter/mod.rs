pub mod double_tap;

use evdev::KeyCode;

pub enum FilterResult {
    Pass,
    Suppress,
}

pub trait KeyFilter {
    /// Called on every key event. Returns whether to pass or suppress.
    fn filter(&mut self, key: KeyCode, value: i32) -> FilterResult;

    fn name(&self) -> &str;
}