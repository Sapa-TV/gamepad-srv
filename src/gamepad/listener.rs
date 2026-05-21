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
        if gamepad_event == GamepadEvent::Ignored {
            return;
        }
        let processed = self.buttons.update(&gamepad_event);
        debug!("Raw event: {:?}", gamepad_event);
        debug!("Processed events: {:?}", processed);
    }

    fn tick(&mut self) {
        let events = self.buttons.tick();
        for event in events {
            debug!("Event: {:?}", event);
        }
    }
}

impl AppInputListener {
    pub fn build() -> Self {
        Self {
            buttons: ButtonDataState::new(),
        }
    }
}
