use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::fs;
use tracing::info;

use super::{Skin, SkinNavigator, SkinViewer};
use crate::{
    config::ConfigInterface,
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
pub struct SkinManager<SCS, CS = ()> {
    skins: Vec<Skin>,
    idx: AtomicUsize,
    skin_change_tx: SCS,
    config: CS,
}

impl<SCS: SkinChangeSender, CS: ConfigInterface> SkinManager<SCS, CS> {
    pub fn builder(skin_change_tx: SCS, config: CS) -> SkinManagerBuilder<SCS, CS> {
        SkinManagerBuilder {
            skin_change_tx,
            config,
        }
    }

    fn cycle_skin(&self, direction: Direction) {
        let skin_list_len = self.skins.len();
        if skin_list_len == 0 {
            return;
        }
        self.idx
            .update(Ordering::SeqCst, Ordering::SeqCst, |current_idx| {
                let next_idx = match direction {
                    Direction::Next => (current_idx + 1) % skin_list_len,
                    Direction::Prev => (current_idx + skin_list_len - 1) % skin_list_len,
                };
                next_idx
            });

        let current_skin = self.current_skin();
        if let Some(current_skin) = current_skin {
            self.skin_change_tx.send_skin_change(current_skin.clone());
            self.config.save_skin(&current_skin.path);
        } else {
            info!("No skins found");
        }
    }
}

impl<SCS: SkinChangeSender, CI: ConfigInterface> SkinNavigator for SkinManager<SCS, CI> {
    fn next_skin(&self) {
        self.cycle_skin(Direction::Next);
    }

    fn prev_skin(&self) {
        self.cycle_skin(Direction::Prev);
    }
}

impl<SCS: SkinChangeSender, CI: ConfigInterface> SkinViewer for SkinManager<SCS, CI> {
    fn current_skin(&self) -> Option<&Skin> {
        let current_idx = self.idx.load(Ordering::SeqCst);
        self.skins.get(current_idx)
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub struct SkinManagerBuilder<SCS, CI> {
    skin_change_tx: SCS,
    config: CI,
}

impl<SCS: SkinChangeSender, CI: ConfigInterface> SkinManagerBuilder<SCS, CI> {
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

    fn find_skin_idx(skins: &[Skin], path: &str) -> usize {
        skins.iter().position(|s| s.path == path).unwrap_or(0)
    }

    pub async fn build(self) -> AppResult<SkinManager<SCS, CI>> {
        let default_skin = self.config.current_skin();
        let skins = Self::load_skins().await?;
        let idx = Self::find_skin_idx(&skins, &default_skin);
        self.config
            .save_skin(skins.get(idx).map(|skin| skin.path.as_str()).unwrap_or(""));

        Ok(SkinManager {
            skins,
            idx: AtomicUsize::new(idx),
            skin_change_tx: self.skin_change_tx,
            config: self.config,
        })
    }
}
