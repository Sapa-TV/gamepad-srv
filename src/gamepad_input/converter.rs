use gilrs::{Axis, Event, EventType};

use crate::constants::AXIS_SCALE;
use crate::gamepad::state::{GamepadEvent, GamepadState};
use crate::skin_switch::buttons::{ButtonEvent, ButtonName};

pub fn button_name(button: gilrs::Button) -> &'static str {
    match button {
        gilrs::Button::South => "A",
        gilrs::Button::East => "B",
        gilrs::Button::North => "Y",
        gilrs::Button::West => "X",
        gilrs::Button::LeftTrigger => "LB",
        gilrs::Button::RightTrigger => "RB",
        gilrs::Button::Select => "SE",
        gilrs::Button::Start => "ST",
        gilrs::Button::LeftThumb => "LS",
        gilrs::Button::RightThumb => "RS",
        gilrs::Button::DPadUp => "DU",
        gilrs::Button::DPadDown => "DD",
        gilrs::Button::DPadLeft => "DL",
        gilrs::Button::DPadRight => "DR",
        gilrs::Button::Mode => "MN",
        gilrs::Button::LeftTrigger2 => "LT",
        gilrs::Button::RightTrigger2 => "RT",
        _ => "U",
    }
}

pub fn gilrs_event_to_button_event(event: &Event) -> Option<ButtonEvent> {
    match event.event {
        EventType::ButtonPressed(btn, _) => {
            let name = match btn {
                gilrs::Button::DPadRight => ButtonName::DPadRight,
                gilrs::Button::DPadLeft => ButtonName::DPadLeft,
                gilrs::Button::Start => ButtonName::Start,
                gilrs::Button::Select => ButtonName::Select,
                _ => return None,
            };
            Some(ButtonEvent::Pressed(name))
        }
        EventType::ButtonReleased(btn, _) => {
            let name = match btn {
                gilrs::Button::DPadRight => ButtonName::DPadRight,
                gilrs::Button::DPadLeft => ButtonName::DPadLeft,
                gilrs::Button::Start => ButtonName::Start,
                gilrs::Button::Select => ButtonName::Select,
                _ => return None,
            };
            Some(ButtonEvent::Released(name))
        }
        _ => None,
    }
}

pub fn process_event(state: &mut GamepadState, event: Event) -> Option<GamepadEvent> {
    match event.event {
        EventType::ButtonPressed(btn, _) => {
            let name = button_name(btn).to_string();
            if !state.buttons.contains(&name) {
                state.buttons.push(name.clone());
                state.buttons.sort();
                return Some(GamepadEvent::ButtonPressed(name));
            }
        }
        EventType::ButtonReleased(btn, _) => {
            let name = button_name(btn).to_string();
            if state.buttons.contains(&name) {
                state.buttons.retain(|b| b != &name);
                return Some(GamepadEvent::ButtonReleased(name));
            }
        }
        EventType::AxisChanged(axis, value, _) => {
            let value = (value * AXIS_SCALE as f32) as i8;
            match axis {
                Axis::LeftStickX => {
                    state.left_x = value;
                }
                Axis::LeftStickY => {
                    state.left_y = value;
                }
                Axis::RightStickX => {
                    state.right_x = value;
                }
                Axis::RightStickY => {
                    state.right_y = value;
                }
                _ => {}
            };
        }
        _ => {}
    }
    None
}
