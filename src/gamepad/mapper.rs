use tracing::debug;

use crate::{
    app::AppActionEnum,
    gamepad::{ButtonEnum, GamepadEvent},
};

pub trait InputMapper: Send + Sync {
    fn map(&self, input: &GamepadEvent) -> AppActionEnum;
}

pub struct AppInputMapper {}

impl InputMapper for AppInputMapper {
    fn map(&self, input: &GamepadEvent) -> AppActionEnum {
        let result = match input {
            GamepadEvent::ButtonHold(ButtonEnum::StartSelect) => AppActionEnum::EnterSkinSelectMode,
            GamepadEvent::ButtonPressed(ButtonEnum::Start)
            | GamepadEvent::ButtonPressed(ButtonEnum::Select) => AppActionEnum::LeaveSkinSelectMode,
            GamepadEvent::ButtonPressed(ButtonEnum::DPadRight) => AppActionEnum::SelectNextSkin,
            GamepadEvent::ButtonPressed(ButtonEnum::DPadLeft) => AppActionEnum::SelectPrevSkin,
            _ => AppActionEnum::None,
        };
        if result != AppActionEnum::None {
            debug!("Mapped input: {:?} -> {:?}", input, result);
        }
        result
    }
}

impl AppInputMapper {
    pub fn new() -> Self {
        Self {}
    }
}
