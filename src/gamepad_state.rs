use gilrs::{Axis, Button, Event, EventType};
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct GamepadOutput {
    pub left_x: f32,
    pub left_y: f32,
    pub right_x: f32,
    pub right_y: f32,
    pub buttons: Vec<String>,
}

pub struct GamepadState {
    pub left_x: f32,
    pub left_y: f32,
    pub right_x: f32,
    pub right_y: f32,
    pub pressed_buttons: HashSet<String>,
}

impl GamepadState {
    pub fn new() -> Self {
        Self {
            left_x: 0.0,
            left_y: 0.0,
            right_x: 0.0,
            right_y: 0.0,
            pressed_buttons: HashSet::new(),
        }
    }

    pub fn to_output(&self) -> GamepadOutput {
        let mut buttons: Vec<String> = self.pressed_buttons.iter().cloned().collect();
        buttons.sort();
        GamepadOutput {
            left_x: self.left_x,
            left_y: self.left_y,
            right_x: self.right_x,
            right_y: self.right_y,
            buttons,
        }
    }
}

fn button_name(button: Button) -> &'static str {
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

pub fn process_event(state: &mut GamepadState, event: Event) {
    match event.event {
        EventType::ButtonPressed(btn, _) => {
            let name = button_name(btn).to_string();
            state.pressed_buttons.insert(name);
        }
        EventType::ButtonReleased(btn, _) => {
            let name = button_name(btn).to_string();
            state.pressed_buttons.remove(&name);
        }
        EventType::AxisChanged(axis, value, _) => {
            match axis {
                Axis::LeftStickX => state.left_x = value,
                Axis::LeftStickY => state.left_y = value,
                Axis::RightStickX => state.right_x = value,
                Axis::RightStickY => state.right_y = value,
                _ => {}
            }
        }
        _ => {}
    }
}