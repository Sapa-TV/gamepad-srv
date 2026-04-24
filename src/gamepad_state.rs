use gilrs::{Axis, Button};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum GamepadEvent {
    ButtonPressed(String),
    ButtonReleased(String),
    AxisChanged { axis: String, value: f32 },
}

#[derive(Debug, Serialize, Clone)]
pub struct GamepadOutput {
    pub left_x: f32,
    pub left_y: f32,
    pub right_x: f32,
    pub right_y: f32,
    pub buttons: Vec<String>,
}

#[derive(Clone)]
pub struct GamepadState {
    pub left_x: f32,
    pub left_y: f32,
    pub right_x: f32,
    pub right_y: f32,
    pub buttons: Vec<String>,
}

impl GamepadState {
    pub fn new() -> Self {
        Self {
            left_x: 0.0,
            left_y: 0.0,
            right_x: 0.0,
            right_y: 0.0,
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

pub fn axis_name(axis: Axis) -> &'static str {
    match axis {
        Axis::LeftStickX => "left_x",
        Axis::LeftStickY => "left_y",
        Axis::RightStickX => "right_x",
        Axis::RightStickY => "right_y",
        _ => "unknown",
    }
}

pub fn button_name(button: Button) -> &'static str {
    match button {
        Button::South => "South",
        Button::East => "East",
        Button::North => "North",
        Button::West => "West",
        Button::LeftTrigger => "LeftTrigger",
        Button::RightTrigger => "RightTrigger",
        Button::Select => "Select",
        Button::Start => "Start",
        Button::LeftThumb => "LeftStick",
        Button::RightThumb => "RightStick",
        Button::DPadUp => "DPadNorth",
        Button::DPadDown => "DPadSouth",
        Button::DPadLeft => "DPadWest",
        Button::DPadRight => "DPadEast",
        Button::Mode => "Menu",
        Button::LeftTrigger2 => "LeftShoulder",
        Button::RightTrigger2 => "RightShoulder",
        _ => "Unknown",
    }
}