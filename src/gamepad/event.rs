use gilrs::EventType;

use crate::gamepad::{buttons::ButtonEnum, sticks::AxisEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamepadEvent {
    ButtonPressed(ButtonEnum),
    ButtonHold(ButtonEnum),
    ButtonReleased(ButtonEnum),
    AxisMoved(AxisEnum),
    Ignored,
}

impl From<gilrs::EventType> for GamepadEvent {
    fn from(event: gilrs::EventType) -> Self {
        use GamepadEvent::*;
        match event {
            EventType::ButtonPressed(raw_button, _) => ButtonPressed(raw_button.into()),
            EventType::ButtonReleased(raw_button, _) => ButtonReleased(raw_button.into()),
            EventType::AxisChanged(_, _, _) => AxisMoved(event.into()),
            _ => Ignored,
        }
    }
}
