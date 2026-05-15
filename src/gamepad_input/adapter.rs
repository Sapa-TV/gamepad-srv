use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::broadcast;
use tracing::{debug, error, info};

use crate::constants::GAMEPAD_POLL_INTERVAL_MS;
use crate::events::AppEvent;
use crate::gamepad::button::ButtonName;
use crate::gamepad::state::{GamepadEvent, GamepadState};
use crate::gamepad_input::converter::{GilrsEventExt, process_axis};
use crate::gamepad_input::port::GamepadInput;

use gilrs::EventType;
use gilrs::Gilrs;

pub struct GilrsAdapter {
    gilrs: Gilrs,
}

impl GilrsAdapter {
    pub fn new() -> Option<Self> {
        Gilrs::new().ok().map(|gilrs| Self { gilrs })
    }
}

impl GamepadInput for GilrsAdapter {
    fn next_button_event(&mut self) -> Option<crate::gamepad::button::ButtonEvent> {
        loop {
            if let Some(event) = self.gilrs.next_event() {
                if let Some(button_event) = event.to_button_event() {
                    return Some(button_event);
                }
            } else {
                return None;
            }
        }
    }
}

pub fn spawn_gilrs_task(
    state: Arc<Mutex<GamepadState>>,
    ws_tx: Arc<broadcast::Sender<GamepadEvent>>,
    events_tx: Arc<broadcast::Sender<AppEvent>>,
) {
    tokio::spawn(async move {
        let mut adapter = match GilrsAdapter::new() {
            Some(a) => a,
            None => {
                error!("Failed to initialize gilrs adapter");
                return;
            }
        };

        info!("Gamepad polling started");

        loop {
            while let Some(event) = adapter.gilrs.next_event() {
                let mut state_guard = state.lock().unwrap();

                match event.event {
                    EventType::ButtonPressed(btn, _) => {
                        let name: ButtonName = btn.into();
                        state_guard.press_button(name);
                        debug!("Button pressed: {:?}", name);
                        let _ = ws_tx.send(GamepadEvent::ButtonPressed(name.to_string().into()));
                    }
                    EventType::ButtonReleased(btn, _) => {
                        let name: ButtonName = btn.into();
                        state_guard.release_button(name);
                        debug!("Button released: {:?}", name);
                        let _ = ws_tx.send(GamepadEvent::ButtonReleased(name.to_string().into()));
                    }
                    EventType::AxisChanged(axis, value, _) => {
                        process_axis(&mut state_guard, axis, value);
                    }
                    _ => {}
                }

                if let Some(button_event) = event.to_button_event() {
                    let _ = events_tx.send(AppEvent::ButtonEvent(button_event));
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(GAMEPAD_POLL_INTERVAL_MS)).await;
        }
    });
}
