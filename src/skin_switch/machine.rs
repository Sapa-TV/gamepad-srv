use std::time::Instant;

use crate::events::AppEvent;
use crate::skin_switch::commands::Command;
use crate::skin_switch::state::{AppSkinState, Direction, SkinChangeState};
use gilrs::EventType;
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

    pub fn handle(&mut self, event: &AppEvent) -> Option<Command> {
        match event {
            AppEvent::Gilrs(gilrs_event) => match gilrs_event.event {
                EventType::ButtonPressed(btn, _) => match btn {
                    gilrs::Button::DPadRight => {
                        if self.state.state == AppSkinState::SkinSwitch {
                            info!("Skin switch: DPadRight pressed, sending direction Right");
                            return Some(Command::SkinChange(Direction::Right));
                        }
                    }
                    gilrs::Button::DPadLeft => {
                        if self.state.state == AppSkinState::SkinSwitch {
                            info!("Skin switch: DPadLeft pressed, sending direction Left");
                            return Some(Command::SkinChange(Direction::Left));
                        }
                    }
                    gilrs::Button::Start => {
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
                    gilrs::Button::Select => {
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
                    _ => {}
                },
                EventType::ButtonReleased(btn, _) => match btn {
                    gilrs::Button::Start => {
                        self.state.start_pressed = false;
                        if self.state.state == AppSkinState::SkinSwitchReady
                            && !self.state.select_pressed
                        {
                            self.state.state = AppSkinState::SkinSwitch;
                            info!("AppSkinState: SkinSwitchReady -> SkinSwitch");
                            return Some(Command::NotifySkinChanging(true));
                        }
                    }
                    gilrs::Button::Select => {
                        self.state.select_pressed = false;
                        if self.state.state == AppSkinState::SkinSwitchReady
                            && !self.state.start_pressed
                        {
                            self.state.state = AppSkinState::SkinSwitch;
                            info!("AppSkinState: SkinSwitchReady -> SkinSwitch");
                            return Some(Command::NotifySkinChanging(true));
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            AppEvent::SkinChange(_) => {}
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

    pub fn state(&self) -> &SkinChangeState {
        &self.state
    }
}
