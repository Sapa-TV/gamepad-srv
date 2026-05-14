use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::time;

use crate::app::Channels;
use crate::button_actions::run_button_actions;
use crate::events::AppEvent;
use crate::gamepad::event_processor::process_event;
use crate::gamepad::state::GamepadEvent;
use crate::skin_manager::discovery::SkinEntry;
use crate::skin_switch::machine::SkinSwitchMachine;
use gilrs::Gilrs;
use tracing::{debug, error, info};

pub fn spawn_stick_tick(
    state: Arc<Mutex<crate::gamepad::state::GamepadState>>,
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
    state: Arc<Mutex<crate::gamepad::state::GamepadState>>,
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

pub fn spawn_button_actions(
    events_rx: broadcast::Receiver<AppEvent>,
    skins: Vec<SkinEntry>,
    current_skin_index: Arc<Mutex<usize>>,
    ws_tx: Arc<broadcast::Sender<GamepadEvent>>,
    save_tx: Arc<std::sync::Mutex<Option<mpsc::Sender<String>>>>,
) {
    tokio::spawn(async move {
        run_button_actions(events_rx, skins, current_skin_index, ws_tx, save_tx).await;
    });
}

pub fn spawn_skin_change_tracker(
    mut events_rx: broadcast::Receiver<AppEvent>,
    events_tx: Arc<broadcast::Sender<AppEvent>>,
    ws_tx: Arc<broadcast::Sender<GamepadEvent>>,
) {
    tokio::spawn(async move {
        let mut machine = SkinSwitchMachine::new();
        let mut next_timeout: Option<tokio::time::Instant> = None;

        loop {
            tokio::select! {
                Ok(event) = events_rx.recv() => {
                    if let Some(cmd) = machine.handle(&event) {
                        match cmd {
                            crate::skin_switch::commands::Command::SkinChange(dir) => {
                                let _ = events_tx.send(AppEvent::SkinChange(dir));
                            }
                            crate::skin_switch::commands::Command::NotifySkinChanging(enabled) => {
                                let _ = ws_tx.send(GamepadEvent::SkinChanging(enabled));
                            }
                            crate::skin_switch::commands::Command::SkinSwitchReady => {
                                let _ = ws_tx.send(GamepadEvent::SkinSwitchReady);
                            }
                        }
                    }
                    next_timeout = machine.deadline();
                }
                _ = async {
                    if let Some(timeout) = next_timeout {
                        tokio::time::sleep_until(timeout).await;
                    }
                }, if next_timeout.is_some() => {
                    if let Some(cmd) = machine.check_timeout() {
                        match cmd {
                            crate::skin_switch::commands::Command::SkinSwitchReady => {
                                let _ = ws_tx.send(GamepadEvent::SkinSwitchReady);
                            }
                            _ => {}
                        }
                    }
                    next_timeout = machine.deadline();
                }
            }
        }
    });
}

impl Channels {
    pub fn spawn_all_tasks(
        &self,
        gilrs_state: Arc<Mutex<crate::gamepad::state::GamepadState>>,
        skins: Vec<SkinEntry>,
        current_skin_index: Arc<Mutex<usize>>,
        save_tx: Arc<std::sync::Mutex<Option<mpsc::Sender<String>>>>,
    ) {
        let ws_tx = self.ws_tx.clone();
        let events_tx = self.events_tx.clone();

        spawn_gilrs_task(gilrs_state, ws_tx.clone(), events_tx.clone());

        let button_events_rx = self.create_events_receiver();
        spawn_button_actions(
            button_events_rx,
            skins,
            current_skin_index,
            ws_tx.clone(),
            save_tx,
        );

        let button_state_events_rx = self.create_events_receiver();
        spawn_skin_change_tracker(button_state_events_rx, events_tx, ws_tx);
    }
}
