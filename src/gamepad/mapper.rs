use tracing::debug;

use crate::{
    app::AppCommandEnum,
    gamepad::{ButtonEnum, CommandReceiver, GamepadEvent},
};

pub trait InputMapper: Send + Sync {
    fn map(&mut self, input: &GamepadEvent) -> AppCommandEnum;
}

pub struct AppInputMapper<CR: CommandReceiver> {
    command_receiver: CR,
}

impl<CR: CommandReceiver> InputMapper for AppInputMapper<CR> {
    fn map(&mut self, input: &GamepadEvent) -> AppCommandEnum {
        let result = match input {
            GamepadEvent::ButtonHold(ButtonEnum::StartSelect) => {
                AppCommandEnum::EnterSkinSelectMode
            }
            GamepadEvent::ButtonPressed(ButtonEnum::Start)
            | GamepadEvent::ButtonPressed(ButtonEnum::Select) => {
                AppCommandEnum::LeaveSkinSelectMode
            }
            GamepadEvent::ButtonPressed(ButtonEnum::DPadRight) => AppCommandEnum::SelectNextSkin,
            GamepadEvent::ButtonPressed(ButtonEnum::DPadLeft) => AppCommandEnum::SelectPrevSkin,
            _ => AppCommandEnum::None,
        };
        if result != AppCommandEnum::None {
            self.command_receiver.receive_command(result);
            // debug!("Mapped input: {:?} -> {:?}", input, result);
        }
        result
    }
}

impl<CR: CommandReceiver> AppInputMapper<CR> {
    pub fn new(command_receiver: CR) -> Self {
        Self { command_receiver }
    }
}
