use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;

use tokio::sync::broadcast;

use crate::events::AppEvent;
use crate::gamepad_state::{GamepadEvent, GamepadState};
use crate::skin::{SkinEntry, discover_skins, load_skin_info};
use crate::skin_change_state::SkinChangeState;
use tracing::{debug, info};

pub struct Channels {
    pub ws_tx: Arc<broadcast::Sender<GamepadEvent>>,
    pub events_tx: Arc<broadcast::Sender<AppEvent>>,
}

impl Channels {
    pub fn new() -> Self {
        let (ws_tx, _) = broadcast::channel(100);
        let (events_tx, _) = broadcast::channel(100);

        Self {
            ws_tx: Arc::new(ws_tx),
            events_tx: Arc::new(events_tx),
        }
    }

    pub fn create_events_receiver(&self) -> broadcast::Receiver<AppEvent> {
        self.events_tx.subscribe()
    }

    pub fn ws_sender(&self) -> Arc<broadcast::Sender<GamepadEvent>> {
        self.ws_tx.clone()
    }
}

#[derive(Clone)]
pub struct AppState {
    pub gamepad_state: Arc<Mutex<GamepadState>>,
    pub button_state: Arc<Mutex<SkinChangeState>>,
    pub ws_tx: Arc<broadcast::Sender<GamepadEvent>>,
    pub shutting_down: Arc<AtomicBool>,
    pub current_skin_index: Arc<Mutex<usize>>,
    pub skins: Vec<SkinEntry>,
}

impl AppState {
    pub fn new(ws_tx: Arc<broadcast::Sender<GamepadEvent>>) -> Self {
        let skins = discover_skins();
        info!("Found {} valid skins", skins.len());

        if !skins.is_empty() {
            match load_skin_info(&skins[0].dir_name) {
                Ok(info) => info!("Default skin: {} (index: 0)", info.name),
                Err(e) => debug!("Failed to load default skin: {}", e),
            }
        } else {
            debug!("No skins found in assets/skins/");
        }

        Self {
            gamepad_state: Arc::new(Mutex::new(GamepadState::new())),
            button_state: Arc::new(Mutex::new(SkinChangeState::default())),
            ws_tx,
            shutting_down: Arc::new(AtomicBool::new(false)),
            current_skin_index: Arc::new(Mutex::new(if !skins.is_empty() {
                0
            } else {
                usize::MAX
            })),
            skins,
        }
    }
}

pub fn create_app_state(ws_tx: Arc<broadcast::Sender<GamepadEvent>>) -> AppState {
    AppState::new(ws_tx)
}
