use tokio::sync::broadcast::Sender;

use crate::{
    app::AppCommandEnum,
    gamepad::GamepadState,
    server::{AppCommandSender, GamepadStateSender, SkinChangeSender, WsInput},
    skins::Skin,
};

pub struct AppWsSender {
    ws_tx: Sender<WsInput>,
}

impl AppWsSender {
    pub fn new(ws_tx: Sender<WsInput>) -> Self {
        Self { ws_tx }
    }
}

impl AppCommandSender for AppWsSender {
    fn send_command(&self, command: AppCommandEnum) {
        self.ws_tx.send(command.into());
    }
}

impl GamepadStateSender for AppWsSender {
    fn send_gamepad_state(&self, state: GamepadState) {
        self.ws_tx.send(state.into());
    }
}

impl SkinChangeSender for AppWsSender {
    fn send_skin_change(&self, skin: Skin) {
        self.ws_tx.send(skin.into());
    }
}
