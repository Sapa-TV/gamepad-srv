use tokio::sync::broadcast::Sender;
use tracing::debug;

use crate::{
    app::AppCommandEnum,
    gamepad::GamepadState,
    server::{AppCommandSender, GamepadStateSender, SkinChangeSender, WsInput},
    skins::Skin,
};

#[derive(Debug, Clone)]
pub struct AppWsSender {
    ws_tx: Sender<WsInput>,
}

impl AppWsSender {
    pub fn new(ws_tx: Sender<WsInput>) -> Self {
        Self { ws_tx }
    }

    fn send(&self, input: WsInput) {
        debug!("Send data to WS worker: {:?}", input);
        self.ws_tx.send(input);
    }
}

impl AppCommandSender for AppWsSender {
    fn send_command(&self, command: AppCommandEnum) {
        self.send(command.into());
    }
}

impl GamepadStateSender for AppWsSender {
    fn send_gamepad_state(&self, state: GamepadState) {
        self.send(state.into());
    }
}

impl SkinChangeSender for AppWsSender {
    fn send_skin_change(&self, skin: Skin) {
        self.send(skin.into());
    }
}
