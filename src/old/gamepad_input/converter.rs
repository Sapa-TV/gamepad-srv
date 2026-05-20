use gilrs::{Axis, Event, EventType};

use crate::constants::AXIS_SCALE;
use crate::gamepad::button::{ButtonEvent, ButtonName};
use crate::gamepad::state::GamepadState;

impl From<gilrs::Button> for ButtonName {
    fn from(btn: gilrs::Button) -> Self {
        match btn {
            gilrs::Button::South => ButtonName::South,
            gilrs::Button::East => ButtonName::East,
            gilrs::Button::North => ButtonName::North,
            gilrs::Button::West => ButtonName::West,
            gilrs::Button::LeftTrigger => ButtonName::LeftBar,
            gilrs::Button::RightTrigger => ButtonName::RightBar,
            gilrs::Button::LeftTrigger2 => ButtonName::LeftTrigger,
            gilrs::Button::RightTrigger2 => ButtonName::RightTrigger,
            gilrs::Button::LeftThumb => ButtonName::LeftStick,
            gilrs::Button::RightThumb => ButtonName::RightStick,
            gilrs::Button::DPadUp => ButtonName::DPadUp,
            gilrs::Button::DPadDown => ButtonName::DPadDown,
            gilrs::Button::DPadLeft => ButtonName::DPadLeft,
            gilrs::Button::DPadRight => ButtonName::DPadRight,
            gilrs::Button::Start => ButtonName::Start,
            gilrs::Button::Select => ButtonName::Select,
            gilrs::Button::Mode => ButtonName::Menu,
            _ => ButtonName::South,
        }
    }
}

pub trait GilrsEventExt {
    fn to_button_event(&self) -> Option<ButtonEvent>;
}

impl GilrsEventExt for Event {
    fn to_button_event(&self) -> Option<ButtonEvent> {
        match self.event {
            EventType::ButtonPressed(btn, _) => Some(ButtonEvent::Pressed(btn.into())),
            EventType::ButtonReleased(btn, _) => Some(ButtonEvent::Released(btn.into())),
            _ => None,
        }
    }
}

pub fn process_axis(state: &mut GamepadState, axis: Axis, value: f32) {
    let value = (value * AXIS_SCALE as f32) as i8;
    match axis {
        Axis::LeftStickX => state.left_x = value,
        Axis::LeftStickY => state.left_y = value,
        Axis::RightStickX => state.right_x = value,
        Axis::RightStickY => state.right_y = value,
        _ => {}
    }
}
