use gamepad::{Button, GamepadState};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SingleGamepadeState {
    left: (f32, f32),
    right: (f32, f32),
    buttons: Vec<String>,
}

fn button_name(button: &Button) -> String {
    let result = match button {
        Button::DPadEast => "DPadEast",
        Button::DPadWest => "DPadWest",
        Button::DPadNorth => "DPadNorth",
        Button::DPadSouth => "DPadSouth",
        Button::East => "East",
        Button::West => "West",
        Button::North => "North",
        Button::South => "South",
        Button::LeftShoulder => "LeftShoulder",
        Button::RightShoulder => "RightShoulder",
        Button::LeftTrigger => "LeftTrigger",
        Button::RightTrigger => "RightTrigger",
        Button::LeftStick => "LeftStick",
        Button::RightStick => "RightStick",
        Button::Menu => "Menu",
        Button::Select => "Select",
        Button::Start => "Start",
    };
    String::from(result)
}

pub fn convert_state(state: &GamepadState) -> SingleGamepadeState {
    let mut buttons: Vec<String> = state
        .buttons()
        .iter()
        .filter(|(_, value)| value.is_pressed())
        .map(|(button, _)| button_name(button))
        .collect();
    buttons.sort();
    SingleGamepadeState {
        left: state.joystick(gamepad::Joystick::Left),
        right: state.joystick(gamepad::Joystick::Right),
        buttons: buttons,
    }
}
