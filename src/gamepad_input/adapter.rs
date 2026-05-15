use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::broadcast;
use tracing::{debug, error, info};

use crate::constants::GAMEPAD_POLL_INTERVAL_MS;
use crate::events::AppEvent;
use crate::gamepad::state::{GamepadEvent, GamepadState};
use crate::gamepad_input::converter::{gilrs_event_to_button_event, process_event};

use gilrs::Gilrs;

pub struct GilrsAdapter {
    gilrs: Gilrs,
}

impl GilrsAdapter {
    pub fn new() -> Option<Self> {
        Gilrs::new().ok().map(|gilrs| Self { gilrs })
    }

    pub fn next_event(&mut self) -> Option<gilrs::Event> {
        self.gilrs.next_event()
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
            while let Some(event) = adapter.next_event() {
                let mut state_guard = state.lock().unwrap();
                if let Some(gamepad_event) = process_event(&mut state_guard, event) {
                    debug!("Gamepad event: {:?}", gamepad_event);
                    let _ = ws_tx.send(gamepad_event);
                }
                if let Some(button_event) = gilrs_event_to_button_event(&event) {
                    let _ = events_tx.send(AppEvent::ButtonEvent(button_event));
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(GAMEPAD_POLL_INTERVAL_MS)).await;
        }
    });
}
