use crate::{app::AppCommandEnum, gamepad::GamepadState, skins::Skin};

#[derive(Debug, Clone)]
pub enum WsInput {
    GamepadState(GamepadState),
    AppCommand(AppCommandEnum),
    SkinSelect(Skin),
}

impl From<AppCommandEnum> for WsInput {
    fn from(command: AppCommandEnum) -> Self {
        WsInput::AppCommand(command)
    }
}

impl From<GamepadState> for WsInput {
    fn from(state: GamepadState) -> Self {
        WsInput::GamepadState(state)
    }
}

impl From<Skin> for WsInput {
    fn from(skin: Skin) -> Self {
        WsInput::SkinSelect(skin)
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
