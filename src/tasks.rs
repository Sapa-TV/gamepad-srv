use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::time;

use crate::app::Channels;
use crate::button_actions::run_button_actions;
use crate::events::AppEvent;
use crate::gamepad::state::{GamepadEvent, GamepadState};
use crate::gamepad_input::adapter::spawn_gilrs_task;
use crate::skin_manager::manager::SkinManager;
use crate::skin_switch::machine::SkinSwitchMachine;

pub fn spawn_stick_tick(
    state: Arc<Mutex<GamepadState>>,
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

pub fn spawn_button_actions(
    events_rx: broadcast::Receiver<AppEvent>,
    skin_manager: Arc<Mutex<SkinManager>>,
    ws_tx: Arc<broadcast::Sender<GamepadEvent>>,
    save_tx: mpsc::Sender<String>,
) {
    tokio::spawn(async move {
        run_button_actions(events_rx, skin_manager, ws_tx, save_tx).await;
    });
}

pub fn spawn_skin_change_tracker(
    mut events_rx: broadcast::Receiver<AppEvent>,
    events_tx: Arc<broadcast::Sender<AppEvent>>,
    ws_tx: Arc<broadcast::Sender<GamepadEvent>>,
) {
    tokio::spawn(async move {
        use crate::skin_switch::commands::Command;

        let mut machine = SkinSwitchMachine::new();
        let mut next_timeout: Option<tokio::time::Instant> = None;

        loop {
            tokio::select! {
                Ok(event) = events_rx.recv() => {
                    if let Some(cmd) = match event {
                        AppEvent::ButtonEvent(button_event) => {
                            machine.handle_button(button_event)
                        }
                        AppEvent::SkinChange(_) => None,
                    } {

                        match cmd {
                            Command::SkinChange(dir) => {
                                let _ = events_tx.send(AppEvent::SkinChange(dir));
                            }
                            Command::NotifySkinChanging(enabled) => {
                                let _ = ws_tx.send(GamepadEvent::SkinChanging(enabled));
                            }
                            Command::SkinSwitchReady => {
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
                            Command::SkinSwitchReady => {
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
        gamepad_state: Arc<Mutex<GamepadState>>,
        skin_manager: Arc<Mutex<SkinManager>>,
        save_tx: mpsc::Sender<String>,
    ) {
        let ws_tx = self.ws_tx.clone();
        let events_tx = self.events_tx.clone();

        let tick_state = gamepad_state.clone();
        spawn_stick_tick(tick_state, ws_tx.clone());

        let gilrs_state = gamepad_state.clone();
        spawn_gilrs_task(gilrs_state, ws_tx.clone(), events_tx.clone());

        let button_events_rx = self.create_events_receiver();
        spawn_button_actions(button_events_rx, skin_manager, ws_tx.clone(), save_tx);

        let button_state_events_rx = self.create_events_receiver();
        spawn_skin_change_tracker(button_state_events_rx, events_tx, ws_tx);
    }
}
