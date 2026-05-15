use crate::constants::SKIN_SWITCH_TIMEOUT_MS;
use crate::gamepad::button::{ButtonEvent, ButtonName};
use crate::skin_switch::commands::Command;
use crate::skin_switch::state::{AppSkinState, SkinChangeState};
use tracing::info;

pub struct SkinSwitchMachine {
    state: SkinChangeState,
}

impl SkinSwitchMachine {
    pub fn new() -> Self {
        Self {
            state: SkinChangeState::default(),
        }
    }

    pub fn handle_button(&mut self, event: ButtonEvent) -> Option<Command> {
        match event {
            ButtonEvent::Pressed(button) => match button {
                ButtonName::DPadRight | ButtonName::DPadLeft => {
                    if self.state.state == AppSkinState::SkinSwitch {
                        info!("Skin switch: {:?} pressed, sending direction", button);
                        return Some(Command::SkinChange(button.into()));
                    }
                }
                ButtonName::Start | ButtonName::Select => {
                    self.state.press_button(button);
                    if self.state.state == AppSkinState::SkinSwitch {
                        self.state.set_normal();
                        info!("AppSkinState: SkinSwitch -> Normal");
                        return Some(Command::NotifySkinChanging(false));
                    }
                    if self.state.state == AppSkinState::Normal
                        && self.state.is_other_pressed(button)
                    {
                        self.state.set_pending();
                        info!("AppSkinState: Normal -> SkinSwitchPending");
                    }
                }
                _ => {}
            },
            ButtonEvent::Released(button) => match button {
                ButtonName::Start | ButtonName::Select => {
                    self.state.release_button(button);
                    if self.state.state == AppSkinState::SkinSwitchPending {
                        self.state.set_normal();
                        info!(
                            "AppSkinState: SkinSwitchPending -> Normal ({:?} released)",
                            button
                        );
                    } else if self.state.state == AppSkinState::SkinSwitchReady
                        && !self.state.is_other_pressed(button)
                    {
                        self.state.set_skin_switch();
                        info!("AppSkinState: SkinSwitchReady -> SkinSwitch");
                        return Some(Command::NotifySkinChanging(true));
                    }
                }
                _ => {}
            },
        }
        None
    }

    pub fn check_timeout(&mut self) -> Option<Command> {
        if self.state.state == AppSkinState::SkinSwitchPending {
            if let Some(pending_since) = self.state.pending_since {
                if pending_since.elapsed()
                    >= std::time::Duration::from_millis(SKIN_SWITCH_TIMEOUT_MS)
                {
                    self.state.set_skin_switch_ready();
                    info!("AppSkinState: SkinSwitchPending -> SkinSwitchReady (timeout)");
                    return Some(Command::SkinSwitchReady);
                }
            }
        }
        None
    }

    pub fn deadline(&self) -> Option<std::time::Instant> {
        if self.state.state == AppSkinState::SkinSwitchPending {
            if let Some(pending_since) = self.state.pending_since {
                return Some(
                    pending_since + std::time::Duration::from_millis(SKIN_SWITCH_TIMEOUT_MS),
                );
            }
        }
        None
    }
}
