use tokio::fs;

use super::{Skin, SkinNavigator, SkinViewer};
use crate::error::{AppError, AppResult};

const SKIN_FOLDER: &str = "assets/skins";

pub enum Direction {
    Next,
    Prev,
}

#[non_exhaustive]
#[derive(Debug)]
pub struct AppSkinManager {
    skins: Vec<Skin>,
    idx: usize,
}

impl AppSkinManager {
    pub fn builder() -> SkinManagerBuilder {
        SkinManagerBuilder::new()
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
    }
}

impl SkinNavigator for AppSkinManager {
    fn next_skin(&mut self) {
        self.cycle_skin(Direction::Next);
    }

    fn prev_skin(&mut self) {
        self.cycle_skin(Direction::Prev);
    }
}

impl SkinViewer for AppSkinManager {
    fn current_skin(&self) -> Option<&Skin> {
        self.skins.get(self.idx)
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub struct SkinManagerBuilder {}

impl SkinManagerBuilder {
    pub fn new() -> Self {
        Self {}
    }

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

    pub async fn build(self) -> AppResult<AppSkinManager> {
        let skin_list = Self::load_skins().await?;
        Ok(AppSkinManager {
            skins: skin_list,
            idx: 0,
        })
    }
}
