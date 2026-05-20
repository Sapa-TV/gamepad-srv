use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;

use tokio::sync::broadcast;

use crate::constants::WS_CHANNEL_CAPACITY;
use crate::events::AppEvent;
use crate::gamepad::state::{GamepadEvent, GamepadState};
use crate::skin_manager::manager::SkinManager;
use tracing::{debug, info};

#[derive(Clone)]
pub struct Channels {
    pub ws_tx: Arc<broadcast::Sender<GamepadEvent>>,
    pub events_tx: Arc<broadcast::Sender<AppEvent>>,
}

impl Channels {
    pub fn new() -> Self {
        let (ws_tx, _) = broadcast::channel(WS_CHANNEL_CAPACITY);
        let (events_tx, _) = broadcast::channel(WS_CHANNEL_CAPACITY);

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
    pub channels: Channels,
    pub gamepad_state: Arc<Mutex<GamepadState>>,
    pub shutting_down: Arc<AtomicBool>,
    pub skin_manager: Arc<Mutex<SkinManager>>,
}

impl AppState {
    pub fn new(channels: Channels, skin_from_config: Option<String>) -> Self {
        let skin_manager = SkinManager::discover_with_config(skin_from_config);
        let count = skin_manager.get_all_skins().len();
        info!("Found {} valid skins", count);

        if let Some(info) = skin_manager.get_current_info() {
            info!("Current skin: {}", info.name);
        } else {
            debug!("No skins found in assets/skins/");
        }

        Self {
            channels,
            gamepad_state: Arc::new(Mutex::new(GamepadState::new())),
            shutting_down: Arc::new(AtomicBool::new(false)),
            skin_manager: Arc::new(Mutex::new(skin_manager)),
        }
    }
}

pub fn create_app_state(channels: Channels, skin_from_config: Option<String>) -> AppState {
    AppState::new(channels, skin_from_config)
}
