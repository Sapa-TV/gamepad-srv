use std::sync::atomic::{AtomicUsize, Ordering};

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
    idx: AtomicUsize,
    skin_shange_tx: SCS,
}

impl<SCS: SkinChangeSender> AppSkinManager<SCS> {
    pub fn builder(skin_shange_tx: SCS) -> SkinManagerBuilder<SCS> {
        SkinManagerBuilder { skin_shange_tx }
    }

    fn cycle_skin(&self, direction: Direction) {
        let skin_list_len = self.skins.len();
        if skin_list_len == 0 {
            return;
        }
        self.idx
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |current_idx| {
                let next_idx = match direction {
                    Direction::Next => (current_idx + 1) % skin_list_len,
                    Direction::Prev => (current_idx + skin_list_len - 1) % skin_list_len,
                };
                Some(next_idx)
            });

        let current_skin = self.current_skin();
        if let Some(current_skin) = current_skin {
            self.skin_shange_tx.send_skin_change(current_skin.clone());
        } else {
            info!("No skins found");
        }
    }
}

impl<SCS: SkinChangeSender> SkinNavigator for AppSkinManager<SCS> {
    fn next_skin(&self) {
        self.cycle_skin(Direction::Next);
    }

    fn prev_skin(&self) {
        self.cycle_skin(Direction::Prev);
    }
}

impl<SCS: SkinChangeSender> SkinViewer for AppSkinManager<SCS> {
    fn current_skin(&self) -> Option<&Skin> {
        let current_idx = self.idx.load(Ordering::SeqCst);
        self.skins.get(current_idx)
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
            idx: AtomicUsize::new(0),
            skin_shange_tx,
        })
    }
}
