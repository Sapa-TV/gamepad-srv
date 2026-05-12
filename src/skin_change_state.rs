use std::time::Instant;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AppSkinState {
    Normal,
    SkinSwitchPending,
    SkinSwitchReady,
    SkinSwitch,
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
