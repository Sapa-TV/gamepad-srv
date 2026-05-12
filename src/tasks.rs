use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use tokio::sync::broadcast;
use tokio::time;

use crate::app::Channels;
use crate::button_actions::{ButtonAction, run_button_actions};
use crate::event_processor::process_event;
use crate::events::AppEvent;
use crate::gamepad_state::GamepadEvent;
use gilrs::Gilrs;
use tracing::{debug, error, info};

pub fn spawn_stick_tick(
    state: Arc<Mutex<crate::gamepad_state::GamepadState>>,
    ws_tx: Arc<broadcast::Sender<GamepadEvent>>,
) {
    tokio::spawn(async move {
        loop {
            time::sleep(Duration::from_millis(50)).await;
            let sticks = {
                let s = state.lock().unwrap();
                GamepadEvent::Sticks {
                    lx: s.left_x,
                    ly: s.left_y,
                    rx: s.right_x,
                    ry: s.right_y,
                }
            };
            let _ = ws_tx.send(sticks);
        }
    });
}

pub fn spawn_gilrs_task(
    state: Arc<Mutex<crate::gamepad_state::GamepadState>>,
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
            tokio::time::sleep(tokio::time::Duration::from_millis(16)).await;
        }
    });
}

pub fn spawn_button_actions(events_rx: broadcast::Receiver<AppEvent>, actions: Vec<ButtonAction>) {
    tokio::spawn(async move {
        run_button_actions(events_rx, actions).await;
    });
}

impl Channels {
    pub fn spawn_all_tasks(&self, gilrs_state: Arc<Mutex<crate::gamepad_state::GamepadState>>) {
        let ws_tx = self.ws_tx.clone();
        let events_tx = self.events_tx.clone();

        spawn_gilrs_task(gilrs_state, ws_tx, events_tx);

        let button_events_rx = self.create_events_receiver();
        spawn_button_actions(button_events_rx, Vec::new());
    }
}
