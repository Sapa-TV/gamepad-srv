use axum::extract::ws::Utf8Bytes;
use serde::Serialize;
use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;

use crate::{app::AppCommandEnum, gamepad::GamepadState, skins::Skin};

#[derive(Debug, Clone)]
pub struct ServerState {
    pub ws_tx: Sender<WsInput>,
    pub shutdown_token: CancellationToken,
}

impl ServerState {
    pub fn new(ws_tx: Sender<WsInput>, shutdown_token: CancellationToken) -> Self {
        Self {
            ws_tx,
            shutdown_token,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum WsInput {
    GamepadState(GamepadState),
    AppCommand { cmd: AppCommandEnum },
    SkinSelect { skin: Skin },
}

impl From<WsInput> for Utf8Bytes {
    fn from(value: WsInput) -> Self {
        let json = serde_json::to_string(&value).unwrap_or_default();
        json.into()
    }
}

impl From<GamepadState> for WsInput {
    fn from(state: GamepadState) -> Self {
        WsInput::GamepadState(state)
    }
}

impl From<AppCommandEnum> for WsInput {
    fn from(command: AppCommandEnum) -> Self {
        WsInput::AppCommand { cmd: command }
    }
}

impl From<Skin> for WsInput {
    fn from(skin: Skin) -> Self {
        WsInput::SkinSelect { skin }
    }
}

pub trait AppCommandSender: Send + Sync + 'static {
    fn send_command(&self, command: AppCommandEnum);
}

pub trait GamepadStateSender: Send + Sync + 'static {
    fn send_gamepad_state(&self, state: GamepadState);
}

pub trait SkinChangeSender: Send + Sync + 'static {
    fn send_skin_change(&self, skin: Skin);
}
