use serde::{Deserialize, Serialize};

use crate::gamepad::button::{ButtonMask, ButtonName};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "t", content = "d")]
pub enum GamepadEvent {
    #[serde(rename = "p")]
    ButtonPressed(String),
    #[serde(rename = "r")]
    ButtonReleased(String),
    #[serde(rename = "s")]
    Sticks { lx: i8, ly: i8, rx: i8, ry: i8 },
    #[serde(rename = "sc")]
    SkinChanged { name: String, path: String },
    #[serde(rename = "sch")]
    SkinChanging(bool),
    #[serde(rename = "ssr")]
    SkinSwitchReady,
}

#[derive(Clone, Serialize)]
pub struct GamepadOutput {
    pub left_x: i8,
    pub left_y: i8,
    pub right_x: i8,
    pub right_y: i8,
    pub buttons: ButtonMask,
}

#[derive(Clone)]
pub struct GamepadState {
    pub left_x: i8,
    pub left_y: i8,
    pub right_x: i8,
    pub right_y: i8,
    pub buttons: ButtonMask,
}

impl GamepadState {
    pub fn new() -> Self {
        Self {
            left_x: 0,
            left_y: 0,
            right_x: 0,
            right_y: 0,
            buttons: ButtonMask(0),
        }
    }

    pub fn press_button(&mut self, name: ButtonName) {
        self.buttons.0 |= name.bit();
    }

    pub fn release_button(&mut self, name: ButtonName) {
        self.buttons.0 &= !name.bit();
    }

    pub fn to_output(&self) -> GamepadOutput {
        GamepadOutput {
            left_x: self.left_x,
            left_y: self.left_y,
            right_x: self.right_x,
            right_y: self.right_y,
            buttons: ButtonMask(self.buttons.0),
        }
    }
}
