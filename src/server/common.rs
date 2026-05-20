use axum::extract::ws::Utf8Bytes;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;

use crate::{app::AppCommand, gamepad::state::GamepadState, skin_manager::manager::SkinManager};

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum WsInput {
    UiState(GamepadState),
    Command { cmd: AppCommand },
}

impl Default for WsInput {
    fn default() -> Self {
        WsInput::UiState(GamepadState::default())
    }
}

impl From<WsInput> for Utf8Bytes {
    fn from(value: WsInput) -> Self {
        let json = serde_json::to_string(&value).unwrap_or_default();
        json.into()
    }
}

impl From<GamepadState> for WsInput {
    fn from(state: GamepadState) -> Self {
        Self::UiState(state)
    }
}

impl From<AppCommand> for WsInput {
    fn from(cmd: AppCommand) -> Self {
        Self::Command { cmd }
    }
}

#[derive(Debug, Clone)]
pub struct ServerState {
    pub skin_manager: Arc<SkinManager>,
    pub ws_tx: Sender<WsInput>,
    pub shutdown_token: CancellationToken,
}

impl ServerState {
    pub fn new(
        skin_manager: Arc<SkinManager>,
        ws_tx: Sender<WsInput>,
        shutdown_token: CancellationToken,
    ) -> Self {
        Self {
            skin_manager,
            ws_tx,
            shutdown_token,
        }
    }
}
