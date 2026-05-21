use tracing::debug;

use super::AppCommandEnum;
use crate::{gamepad::CommandReceiver, skins::SkinNavigator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppMode {
    Normal,
    SkinSelect,
}

pub struct AppState<SN> {
    skin_manager: SN,
    current_mode: AppMode,
}

impl<SN: SkinNavigator> AppState<SN> {
    pub fn new(skin_manager: SN) -> Self {
        Self {
            skin_manager,
            current_mode: AppMode::Normal,
        }
    }
}

impl<SN: SkinNavigator> CommandReceiver for AppState<SN> {
    fn receive_command(&mut self, command: AppCommandEnum) {
        debug!("Received command: {:?}", command);
        match command {
            AppCommandEnum::EnterSkinSelectMode => {
                self.current_mode = AppMode::SkinSelect;
            }
            AppCommandEnum::LeaveSkinSelectMode => {
                self.current_mode = AppMode::Normal;
            }
            AppCommandEnum::SelectNextSkin => {
                if self.current_mode == AppMode::SkinSelect {
                    self.skin_manager.next_skin();
                }
            }
            AppCommandEnum::SelectPrevSkin => {
                if self.current_mode == AppMode::SkinSelect {
                    self.skin_manager.next_skin();
                }
            }
            _ => {}
        }
    }
}
