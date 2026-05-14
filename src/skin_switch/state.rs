use std::time::Instant;

use crate::skin_switch::buttons::ButtonName;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AppSkinState {
    Normal,
    SkinSwitchPending,
    SkinSwitchReady,
    SkinSwitch,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Right,
}

impl From<ButtonName> for Direction {
    fn from(button: ButtonName) -> Self {
        match button {
            ButtonName::DPadRight => Direction::Right,
            ButtonName::DPadLeft => Direction::Left,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub struct SkinChangeState {
    pub state: AppSkinState,
    pub start_pressed: bool,
    pub select_pressed: bool,
    pub pending_since: Option<Instant>,
}

impl Default for SkinChangeState {
    fn default() -> Self {
        Self {
            state: AppSkinState::Normal,
            start_pressed: false,
            select_pressed: false,
            pending_since: None,
        }
    }
}
