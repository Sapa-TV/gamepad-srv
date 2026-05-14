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

impl SkinChangeState {
    pub fn press_button(&mut self, button: ButtonName) {
        match button {
            ButtonName::Start => self.start_pressed = true,
            ButtonName::Select => self.select_pressed = true,
            _ => {}
        }
    }

    pub fn release_button(&mut self, button: ButtonName) {
        match button {
            ButtonName::Start => self.start_pressed = false,
            ButtonName::Select => self.select_pressed = false,
            _ => {}
        }
    }

    pub fn is_other_pressed(&self, button: ButtonName) -> bool {
        match button {
            ButtonName::Start => self.select_pressed,
            ButtonName::Select => self.start_pressed,
            _ => false,
        }
    }

    pub fn set_pending(&mut self) {
        self.state = AppSkinState::SkinSwitchPending;
        self.pending_since = Some(Instant::now());
    }

    pub fn clear_pending(&mut self) {
        self.pending_since = None;
    }

    pub fn set_normal(&mut self) {
        self.state = AppSkinState::Normal;
        self.clear_pending();
    }

    pub fn set_skin_switch_ready(&mut self) {
        self.state = AppSkinState::SkinSwitchReady;
        self.clear_pending();
    }

    pub fn set_skin_switch(&mut self) {
        self.state = AppSkinState::SkinSwitch;
    }
}
