use tokio::fs;
use tracing::info;

use super::{Skin, SkinNavigator, SkinViewer};
use crate::{
    error::{AppError, AppResult},
    server::SkinChangeSender,
};

const SKIN_FOLDER: &str = "assets/skins";

pub enum Direction {
    Next,
    Prev,
}

#[non_exhaustive]
#[derive(Debug)]
pub struct AppSkinManager<SCS> {
    skins: Vec<Skin>,
    idx: usize,
    skin_shange_tx: SCS,
}

impl<SCS: SkinChangeSender> AppSkinManager<SCS> {
    pub fn builder(skin_shange_tx: SCS) -> SkinManagerBuilder<SCS> {
        SkinManagerBuilder { skin_shange_tx }
    }

    fn cycle_skin(&mut self, direction: Direction) {
        let skin_list_len = self.skins.len();
        match skin_list_len {
            0 => return,
            len => {
                self.idx = match direction {
                    Direction::Next => (self.idx + 1) % len,
                    Direction::Prev => (self.idx + len - 1) % len,
                };
            }
        }
        let current_skin = self.current_skin();
        if let Some(current_skin) = current_skin {
            self.skin_shange_tx.send_skin_change(current_skin.clone());
        } else {
            info!("No skins found");
        }
    }
}

impl<SCS: SkinChangeSender> SkinNavigator for AppSkinManager<SCS> {
    fn next_skin(&mut self) {
        self.cycle_skin(Direction::Next);
    }

    fn prev_skin(&mut self) {
        self.cycle_skin(Direction::Prev);
    }
}

impl<SCS: SkinChangeSender> SkinViewer for AppSkinManager<SCS> {
    fn current_skin(&self) -> Option<&Skin> {
        self.skins.get(self.idx)
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub struct SkinManagerBuilder<SCS> {
    skin_shange_tx: SCS,
}

impl<SCS: SkinChangeSender> SkinManagerBuilder<SCS> {
    async fn load_skins() -> AppResult<Vec<Skin>> {
        let mut skin_list = Vec::new();
        let mut entries = fs::read_dir(SKIN_FOLDER)
            .await
            .map_err(|err| AppError::Skin(format!("Skin directory read error: {err}")))?;

        while let Some(entry) = entries.next_entry().await? {
            let new_skin = Skin::try_from_dir(&entry.path()).ok();
            if let Some(skin) = new_skin {
                skin_list.push(skin);
            }
        }

        Ok(skin_list)
    }

    pub async fn build(self) -> AppResult<AppSkinManager<SCS>> {
        let skin_list = Self::load_skins().await?;
        let skin_shange_tx = self.skin_shange_tx;
        Ok(AppSkinManager {
            skins: skin_list,
            idx: 0,
            skin_shange_tx,
        })
    }
}
