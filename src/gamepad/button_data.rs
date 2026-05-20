use std::time::Instant;
use strum::EnumCount;
use tracing::debug;

use crate::gamepad::{buttons::ButtonEnum, event::GamepadEvent};

#[non_exhaustive]
#[derive(Debug, Default, Clone)]
struct ButtonData {
    pressed: bool,
    hold_triggered: bool,
    hold_time: Option<Instant>,
}

#[non_exhaustive]
pub struct ButtonDataState {
    inner: Vec<ButtonData>,
}

const BUTTONS_COUNT: usize = ButtonEnum::COUNT;

impl ButtonDataState {
    pub fn new() -> Self {
        let inner: Vec<ButtonData> = vec![ButtonData::default(); BUTTONS_COUNT];
        Self { inner }
    }

    pub fn update(&mut self, event: &GamepadEvent) {
        match *event {
            GamepadEvent::ButtonPressed(button) => {
                let button_data = &mut self.inner[button as usize];
                button_data.pressed = true;
                button_data.hold_time = Some(Instant::now());
            }
            GamepadEvent::ButtonReleased(button) => {
                let button_data = &mut self.inner[button as usize];
                button_data.pressed = false;
                button_data.hold_time = None;
            }
            _ => {}
        }
    }
}
