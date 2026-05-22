use tokio::sync::broadcast::Sender;
use tracing::error;

use crate::{
    app::AppCommandEnum,
    gamepad::GamepadState,
    server::{AppCommandSender, GamepadStateSender, SkinChangeSender, WsInput},
    skins::Skin,
};

#[derive(Debug, Clone)]
pub struct WsSender {
    ws_tx: Sender<WsInput>,
}

impl WsSender {
    pub fn new(ws_tx: Sender<WsInput>) -> Self {
        Self { ws_tx }
    }

    fn send(&self, input: WsInput) {
        if let Err(err) = self.ws_tx.send(input) {
            error!("Error sending ws input: {err}");
        }
    }
}

impl AppCommandSender for WsSender {
    fn send_command(&self, command: AppCommandEnum) {
        self.send(command.into());
    }
}

impl GamepadStateSender for WsSender {
    fn send_gamepad_state(&self, state: GamepadState) {
        self.send(state.into());
    }
}

impl SkinChangeSender for WsSender {
    fn send_skin_change(&self, skin: Skin) {
        self.send(skin.into());
    }
}
