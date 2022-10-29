use std::collections::HashMap;

use crate::{ScanCode, KeyCode};

#[derive(Debug, Clone, Default)]
pub struct Input {
    pub(crate) scancodes: HashMap<ScanCode, bool>,
    pub(crate) keycodes: HashMap<KeyCode, bool>,

    pub(crate) scancodes_this_frame: HashMap<ScanCode, bool>,
    pub(crate) keycodes_this_frame: HashMap<KeyCode, bool>,

}

impl Input {
    pub fn is_scancode_pressed(&self, scancode: ScanCode) -> bool {
        self.scancodes.get(&scancode).unwrap_or(&false).to_owned()
    }

    pub fn is_key_pressed(&self, keycode: KeyCode) -> bool {
        self.keycodes.get(&keycode).unwrap_or(&false).to_owned()
    }

    pub fn is_scancode_pressed_this_frame(&self, scancode: ScanCode) -> bool {
        self.scancodes_this_frame.get(&scancode).unwrap_or(&false).to_owned()
    }

    pub fn is_scancode_released_this_frame(&self, scancode: ScanCode) -> bool {
        !self.scancodes_this_frame.get(&scancode).unwrap_or(&true).to_owned()
    }

    pub fn is_key_pressed_this_frame(&self, keycode: KeyCode) -> bool {
        self.keycodes_this_frame.get(&keycode).unwrap_or(&false).to_owned()
    }

    pub fn is_key_released_this_frame(&self, keycode: KeyCode) -> bool {
        !self.keycodes_this_frame.get(&keycode).unwrap_or(&true).to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct Time {
    pub frames: u64,
    pub delta_time: std::time::Duration,
    pub frame_time: std::time::Instant 
}

impl Default for Time {
    fn default() -> Self {
        Time {
            frames: 0,
            delta_time: std::time::Duration::from_secs(0),
            frame_time: std::time::Instant::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Display {
    pub resolution: (u32, u32),
    pub position: (i32, i32)
}

impl Display {
    pub fn aspect_ratio(&self) -> f64 {
        self.resolution.0 as f64 / self.resolution.1 as f64
    }
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub input: Input,
    pub time: Time,
    pub display: Display,
}