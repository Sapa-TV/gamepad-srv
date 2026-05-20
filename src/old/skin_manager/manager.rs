use tokio::fs;

use crate::{
    error::{AppError, AppResult},
    skin_manager::skin::Skin,
};

const SKIN_DIR: &str = "assets/skins";

pub enum Direction {
    Next,
    Prev,
}

#[non_exhaustive]
#[derive(Debug)]
pub struct SkinManager {
    skins: Vec<Skin>,
    idx: usize,
}

impl SkinManager {
    pub fn builder() -> SkinManagerBuilder {
        SkinManagerBuilder::default()
    }

    pub fn get_current_skin(&self) -> Option<&Skin> {
        self.skins.get(self.idx)
    }

    // pub fn get_skins(&self) -> Vec<&Skin> {
    //     self.skins.iter().collect()
    // }

    pub fn cycle_skin(&mut self, direction: Direction) -> Option<&Skin> {
        let skin_list_len = self.skins.len();
        match skin_list_len {
            0 => return None,
            len => {
                self.idx = match direction {
                    Direction::Next => (self.idx + 1) % len,
                    Direction::Prev => (self.idx + len - 1) % len,
                };
                self.get_current_skin()
            }
        }
    }

    pub fn next_skin(&mut self) -> Option<&Skin> {
        self.cycle_skin(Direction::Next)
    }

    pub fn prev_skin(&mut self) -> Option<&Skin> {
        self.cycle_skin(Direction::Prev)
    }
}

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct SkinManagerBuilder {}

impl SkinManagerBuilder {
    async fn load_skins() -> AppResult<Vec<Skin>> {
        let mut skin_list = Vec::new();
        let mut entries = fs::read_dir(SKIN_DIR)
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

    pub async fn build(self) -> AppResult<SkinManager> {
        let skin_list = Self::load_skins().await?;
        Ok(SkinManager {
            skins: skin_list,
            idx: 0,
        })
    }
}
