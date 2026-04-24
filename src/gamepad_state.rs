use gilrs::Button;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "t", content = "d")]
pub enum GamepadEvent {
    #[serde(rename = "p")]
    ButtonPressed(String),
    #[serde(rename = "r")]
    ButtonReleased(String),
    #[serde(rename = "s")]
    Sticks { lx: i8, ly: i8, rx: i8, ry: i8 },
}

#[derive(Debug, Serialize, Clone)]
pub struct GamepadOutput {
    #[serde(rename = "lx")]
    pub left_x: i8,
    #[serde(rename = "ly")]
    pub left_y: i8,
    #[serde(rename = "rx")]
    pub right_x: i8,
    #[serde(rename = "ry")]
    pub right_y: i8,
    pub buttons: Vec<String>,
}

#[derive(Clone)]
pub struct GamepadState {
    pub left_x: i8,
    pub left_y: i8,
    pub right_x: i8,
    pub right_y: i8,
    pub buttons: Vec<String>,
}

impl GamepadState {
    pub fn new() -> Self {
        Self {
            left_x: 0,
            left_y: 0,
            right_x: 0,
            right_y: 0,
            buttons: Vec::new(),
        }
    }

    pub fn to_output(&self) -> GamepadOutput {
        GamepadOutput {
            left_x: self.left_x,
            left_y: self.left_y,
            right_x: self.right_x,
            right_y: self.right_y,
            buttons: self.buttons.clone(),
        }
    }
}

pub fn button_name(button: Button) -> &'static str {
    match button {
        Button::South => "A",
        Button::East => "B",
        Button::North => "Y",
        Button::West => "X",
        Button::LeftTrigger => "LB",
        Button::RightTrigger => "RB",
        Button::Select => "SE",
        Button::Start => "ST",
        Button::LeftThumb => "LS",
        Button::RightThumb => "RS",
        Button::DPadUp => "DU",
        Button::DPadDown => "DD",
        Button::DPadLeft => "DL",
        Button::DPadRight => "DR",
        Button::Mode => "MN",
        Button::LeftTrigger2 => "LT",
        Button::RightTrigger2 => "RT",
        _ => "U",
    }
}