use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;

use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::time;

use crate::app::Channels;
use crate::button_actions::{ButtonAction, run_button_actions};
use crate::event_processor::process_event;
use crate::events::AppEvent;
use crate::gamepad_state::GamepadEvent;
use crate::skin::SkinEntry;
use crate::skin_change_state::{AppSkinState, SkinChangeState};
use gilrs::{Button, Gilrs};
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

pub fn spawn_button_actions(
    events_rx: broadcast::Receiver<AppEvent>,
    actions: Vec<ButtonAction>,
    skins: Vec<SkinEntry>,
    current_skin_index: Arc<Mutex<usize>>,
    ws_tx: Arc<broadcast::Sender<GamepadEvent>>,
    save_tx: Arc<std::sync::Mutex<Option<mpsc::Sender<String>>>>,
) {
    tokio::spawn(async move {
        run_button_actions(
            events_rx,
            actions,
            skins,
            current_skin_index,
            ws_tx,
            save_tx,
        )
        .await;
    });
}

pub fn spawn_skin_change_tracker(
    button_state: Arc<Mutex<SkinChangeState>>,
    mut events_rx: broadcast::Receiver<AppEvent>,
    events_tx: Arc<broadcast::Sender<AppEvent>>,
    ws_tx: Arc<broadcast::Sender<GamepadEvent>>,
) {
    tokio::spawn(async move {
        use gilrs::EventType;
        loop {
            tokio::select! {
                Ok(AppEvent::Gilrs(event)) = events_rx.recv() => {
                    let mut state = button_state.lock().unwrap();
                    match event.event {
                        EventType::ButtonPressed(btn, _) => match btn {
                            Button::DPadRight => {
                                if state.state == AppSkinState::SkinSwitch {
                                    debug!("Skin switch: DPadRight pressed, sending direction Right");
                                    let _ = events_tx.send(AppEvent::SkinChange(crate::skin_change_state::Direction::Right));
                                }
                            }
                            Button::DPadLeft => {
                                if state.state == AppSkinState::SkinSwitch {
                                    debug!("Skin switch: DPadLeft pressed, sending direction Left");
                                    let _ = events_tx.send(AppEvent::SkinChange(crate::skin_change_state::Direction::Left));
                                }
                            }
                            Button::Start => {
                                state.start_pressed = true;
                                if state.state == AppSkinState::SkinSwitch {
                                    state.state = AppSkinState::Normal;
                                    let _ = ws_tx.send(GamepadEvent::SkinChanging(false));
                                    info!("AppSkinState: SkinSwitch -> Normal");
                                }
                                if state.state == AppSkinState::Normal && state.select_pressed {
                                    state.state = AppSkinState::SkinSwitchPending;
                                    state.pending_since = Some(Instant::now());
                                    info!("AppSkinState: Normal -> SkinSwitchPending");
                                }
                            }
                            Button::Select => {
                                state.select_pressed = true;
                                if state.state == AppSkinState::SkinSwitch {
                                    state.state = AppSkinState::Normal;
                                    let _ = ws_tx.send(GamepadEvent::SkinChanging(false));
                                    info!("AppSkinState: SkinSwitch -> Normal");
                                }
                                if state.state == AppSkinState::Normal && state.start_pressed {
                                    state.state = AppSkinState::SkinSwitchPending;
                                    state.pending_since = Some(Instant::now());
                                    info!("AppSkinState: Normal -> SkinSwitchPending");
                                }
                            }
                            _ => {}
                        },
                        EventType::ButtonReleased(btn, _) => match btn {
                            Button::Start => {
                                state.start_pressed = false;
                                if state.state == AppSkinState::SkinSwitchReady
                                    && !state.select_pressed
                                {
                                    state.state = AppSkinState::SkinSwitch;
                                    let _ = ws_tx.send(GamepadEvent::SkinChanging(true));
                                    info!("AppSkinState: SkinSwitchReady -> SkinSwitch");
                                }
                            }
                            Button::Select => {
                                state.select_pressed = false;
                                if state.state == AppSkinState::SkinSwitchReady
                                    && !state.start_pressed
                                {
                                    state.state = AppSkinState::SkinSwitch;
                                    let _ = ws_tx.send(GamepadEvent::SkinChanging(true));
                                    info!("AppSkinState: SkinSwitchReady -> SkinSwitch");
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
                _ = time::sleep(Duration::from_millis(100)) => {
                    let state_guard = button_state.lock().unwrap();
                    if state_guard.state == AppSkinState::SkinSwitchPending {
                        if let Some(pending_since) = state_guard.pending_since {
                            if pending_since.elapsed() >= Duration::from_secs(2) {
                                drop(state_guard);
                                let mut state = button_state.lock().unwrap();
                                state.state = AppSkinState::SkinSwitchReady;
                                state.pending_since = None;
                                info!("AppSkinState: SkinSwitchPending -> SkinSwitchReady (timeout)");
                                let _ = ws_tx.send(GamepadEvent::SkinSwitchReady);
                            }
                        }
                    }
                }
            }
        }
    });
}

impl Channels {
    pub fn spawn_all_tasks(
        &self,
        gilrs_state: Arc<Mutex<crate::gamepad_state::GamepadState>>,
        button_state: Arc<Mutex<SkinChangeState>>,
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
            Vec::new(),
            skins,
            current_skin_index,
            ws_tx.clone(),
            save_tx,
        );

        let button_state_events_rx = self.create_events_receiver();
        spawn_skin_change_tracker(button_state, button_state_events_rx, events_tx, ws_tx);
    }
}
