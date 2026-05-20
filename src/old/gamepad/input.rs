use gilrs::Gilrs;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::broadcast;
use tokio::time;
use tracing::{debug, error, info};

use crate::events::AppEvent;
use crate::gamepad::event_processor::process_event;
use crate::gamepad::state::{GamepadEvent, GamepadState};

pub fn spawn_gilrs_task(
    state: Arc<Mutex<GamepadState>>,
    ws_tx: Arc<broadcast::Sender<GamepadEvent>>,
    events_tx: Arc<broadcast::Sender<AppEvent>>,
) {
    tokio::spawn(async move {
        let mut gilrs = match Gilrs::new() {
            Ok(g) => g,
            Err(e) => {
                error!("Failed to initialize gilrs: {}", e);
                return;
            }
        };

        info!("Gamepad polling started");

        loop {
            while let Some(event) = gilrs.next_event() {
                let mut state_guard = state.lock().unwrap();
                if let Some(gamepad_event) = process_event(&mut state_guard, event) {
                    debug!("Gamepad event: {:?}", gamepad_event);
                    let _ = ws_tx.send(gamepad_event);
                }
                let _ = events_tx.send(AppEvent::Gilrs(event));
            }
            time::sleep(time::Duration::from_millis(16)).await;
        }
    });
}
