use tracing::debug;

use super::AppCommandEnum;
use crate::{gamepad::CommandReceiver, server::AppCommandSender, skins::SkinNavigator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppMode {
    Normal,
    SkinSelect,
}

pub struct AppState<SN, ACS> {
    skin_manager: SN,
    current_mode: AppMode,
    command_tx: ACS,
}

impl<SN: SkinNavigator, ACS: AppCommandSender> AppState<SN, ACS> {
    pub fn new(skin_manager: SN, command_tx: ACS) -> Self {
        Self {
            skin_manager,
            current_mode: AppMode::Normal,
            command_tx,
        }
    }
}

impl<SN: SkinNavigator, ACS: AppCommandSender> CommandReceiver for AppState<SN, ACS> {
    fn receive_command(&mut self, command: AppCommandEnum) {
        debug!("Received command: {:?}", command);
        match command {
            AppCommandEnum::EnterSkinSelectMode => {
                self.current_mode = AppMode::SkinSelect;
                self.command_tx
                    .send_command(AppCommandEnum::EnterSkinSelectMode);
            }
            AppCommandEnum::LeaveSkinSelectMode => {
                self.current_mode = AppMode::Normal;
                self.command_tx
                    .send_command(AppCommandEnum::LeaveSkinSelectMode);
            }
            AppCommandEnum::SelectNextSkin => {
                if self.current_mode == AppMode::SkinSelect {
                    self.skin_manager.next_skin();
                }
            }
            AppCommandEnum::SelectPrevSkin => {
                if self.current_mode == AppMode::SkinSelect {
                    self.skin_manager.prev_skin();
                }
            }
            _ => {}
        }
    }
}
