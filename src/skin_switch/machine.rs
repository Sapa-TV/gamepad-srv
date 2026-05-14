use std::time::Instant;

use crate::skin_switch::buttons::{ButtonEvent, ButtonName};
use crate::skin_switch::commands::Command;
use crate::skin_switch::state::{AppSkinState, Direction, SkinChangeState};
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
            ButtonEvent::Pressed(name) => match name {
                ButtonName::DPadRight => {
                    if self.state.state == AppSkinState::SkinSwitch {
                        info!("Skin switch: DPadRight pressed, sending direction Right");
                        return Some(Command::SkinChange(Direction::Right));
                    }
                }
                ButtonName::DPadLeft => {
                    if self.state.state == AppSkinState::SkinSwitch {
                        info!("Skin switch: DPadLeft pressed, sending direction Left");
                        return Some(Command::SkinChange(Direction::Left));
                    }
                }
                ButtonName::Start => {
                    self.state.start_pressed = true;
                    if self.state.state == AppSkinState::SkinSwitch {
                        self.state.state = AppSkinState::Normal;
                        info!("AppSkinState: SkinSwitch -> Normal");
                        return Some(Command::NotifySkinChanging(false));
                    }
                    if self.state.state == AppSkinState::Normal && self.state.select_pressed {
                        self.state.state = AppSkinState::SkinSwitchPending;
                        self.state.pending_since = Some(Instant::now());
                        info!("AppSkinState: Normal -> SkinSwitchPending");
                    }
                }
                ButtonName::Select => {
                    self.state.select_pressed = true;
                    if self.state.state == AppSkinState::SkinSwitch {
                        self.state.state = AppSkinState::Normal;
                        info!("AppSkinState: SkinSwitch -> Normal");
                        return Some(Command::NotifySkinChanging(false));
                    }
                    if self.state.state == AppSkinState::Normal && self.state.start_pressed {
                        self.state.state = AppSkinState::SkinSwitchPending;
                        self.state.pending_since = Some(Instant::now());
                        info!("AppSkinState: Normal -> SkinSwitchPending");
                    }
                }
            },
            ButtonEvent::Released(name) => match name {
                ButtonName::Start => {
                    self.state.start_pressed = false;
                    if self.state.state == AppSkinState::SkinSwitchPending {
                        self.state.state = AppSkinState::Normal;
                        self.state.pending_since = None;
                        info!("AppSkinState: SkinSwitchPending -> Normal (Start released)");
                    } else if self.state.state == AppSkinState::SkinSwitchReady
                        && !self.state.select_pressed
                    {
                        self.state.state = AppSkinState::SkinSwitch;
                        info!("AppSkinState: SkinSwitchReady -> SkinSwitch");
                        return Some(Command::NotifySkinChanging(true));
                    }
                }
                ButtonName::Select => {
                    self.state.select_pressed = false;
                    if self.state.state == AppSkinState::SkinSwitchPending {
                        self.state.state = AppSkinState::Normal;
                        self.state.pending_since = None;
                        info!("AppSkinState: SkinSwitchPending -> Normal (Select released)");
                    } else if self.state.state == AppSkinState::SkinSwitchReady
                        && !self.state.start_pressed
                    {
                        self.state.state = AppSkinState::SkinSwitch;
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
                if pending_since.elapsed() >= std::time::Duration::from_secs(2) {
                    self.state.state = AppSkinState::SkinSwitchReady;
                    self.state.pending_since = None;
                    info!("AppSkinState: SkinSwitchPending -> SkinSwitchReady (timeout)");
                    return Some(Command::SkinSwitchReady);
                }
            }
        }
        None
    }

    pub fn deadline(&self) -> Option<tokio::time::Instant> {
        if self.state.state == AppSkinState::SkinSwitchPending {
            if let Some(pending_since) = self.state.pending_since {
                return Some((pending_since + std::time::Duration::from_secs(2)).into());
            }
        }
        None
    }
}
