use serde::Serialize;
use tracing::debug;

use crate::gamepad::{
    buttons::{ButtonEnum, Buttons},
    event::GamepadEvent,
    sticks::{AxisEnum, Stick},
};

pub trait GamepadState: Send + Sync {
    fn update(&mut self, event: &GamepadEvent);
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct AppGamepadState {
    #[serde(rename = "ls")]
    pub left_stick: Stick,
    #[serde(rename = "rs")]
    pub right_stick: Stick,
    #[serde(rename = "b")]
    pub buttons: Buttons,
}

impl AppGamepadState {
    pub fn new() -> Self {
        Self {
            left_stick: Stick::new(),
            right_stick: Stick::new(),
            buttons: Buttons::new(),
        }
    }

    fn stick_update(&mut self, axis: &AxisEnum) {
        match axis {
            AxisEnum::LeftStickX(value) => self.left_stick.update_x(*value),
            AxisEnum::LeftStickY(value) => self.left_stick.update_y(*value),
            AxisEnum::RightStickX(value) => self.right_stick.update_x(*value),
            AxisEnum::RightStickY(value) => self.right_stick.update_y(*value),
            _ => {}
        }
    }

    fn button_press(&mut self, button: &ButtonEnum) {
        self.buttons.press(button);
    }

    fn button_release(&mut self, button: &ButtonEnum) {
        self.buttons.release(button);
    }
}

impl GamepadState for AppGamepadState {
    fn update(&mut self, event: &GamepadEvent) {
        debug!("Gamepad event: {:?}", event);
        match event {
            GamepadEvent::ButtonPressed(button) => self.button_press(button),
            GamepadEvent::ButtonReleased(button) => self.button_release(button),
            GamepadEvent::AxisMoved(axis_change) => self.stick_update(axis_change),
            _ => {}
        }
    }
}
