use tracing::debug;

use crate::gamepad::{ButtonDataState, GamepadEvent};

pub trait InputListener: Send + Sync {
    fn handle_raw(&mut self, event: gilrs::Event);
    fn tick(&mut self);
}

#[non_exhaustive]
pub struct AppInputListener {
    buttons: ButtonDataState,
}

impl InputListener for AppInputListener {
    fn handle_raw(&mut self, raw_event: gilrs::Event) {
        // TODO: Implement application input converter
        let gamepad_event: GamepadEvent = raw_event.into();
        debug!("Raw event: {:?}", gamepad_event);
        self.buttons.update(&gamepad_event);
    }

    fn tick(&mut self) {
        debug!("Listener tick");
    }
}

impl AppInputListener {
    pub fn build() -> Self {
        Self {
            buttons: ButtonDataState::new(),
        }
    }
}
