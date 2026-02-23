use std::collections::HashMap;
use std::time::{Duration, Instant};
use evdev::{KeyCode};
use crate::filter::{FilterResult, KeyFilter};

pub struct DoubleTapFilter {
    press_map: HashMap<u16, Instant>,
    key_pair: (u16, u16),
    delay: Duration
}

impl DoubleTapFilter {
    pub fn new(key_pair: (u16, u16), delay: Duration) -> DoubleTapFilter {
        let press_map: HashMap<u16, Instant> = HashMap::new();
        DoubleTapFilter { press_map, key_pair, delay }
    }

    fn get_opposite_key(&self, pressed_key: u16) -> Option<u16> {
        let (keycode_1, keycode_2) = self.key_pair;
        match pressed_key {
            key if key == keycode_1 => Some(keycode_2),
            key if key == keycode_2 => Some(keycode_1),
            _ => None
        }
    }
}

impl KeyFilter for DoubleTapFilter {
    fn filter(&mut self, key: KeyCode, value: i32) -> FilterResult {
        // If it's not a press return ok
        if value != 1 {
            return FilterResult::Pass;
        }

        let now = Instant::now();
        let should_supress = self.get_opposite_key(key.code())
            .and_then( |key_code| self.press_map.get(&key_code))
            .is_some_and(|inst| now - *inst < self.delay);

        self.press_map.insert(key.code(), now);
        if should_supress {
            return FilterResult::Suppress
        }
        FilterResult::Pass

    }

    fn name(&self) -> &str {
        "DoubleTapFilter"
    }
}