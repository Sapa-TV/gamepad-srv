use super::discovery::{SkinEntry, discover_skins, load_skin_info};
use crate::skin_switch::state::Direction;
use tracing::info;

#[derive(Clone)]
pub struct SkinManager {
    skins: Vec<SkinEntry>,
    current_idx: usize,
}

impl SkinManager {
    pub fn discover() -> Self {
        let skins = discover_skins();
        Self {
            skins,
            current_idx: 0,
        }
    }

    pub fn with_skins(skins: Vec<SkinEntry>) -> Self {
        Self {
            skins,
            current_idx: 0,
        }
    }

    pub fn discover_with_config(skin_from_config: Option<String>) -> Self {
        let skins = discover_skins();
        let current_idx = if !skins.is_empty() {
            if let Some(name) = &skin_from_config {
                if let Some(idx) = skins.iter().position(|s| &s.dir_name == name) {
                    info!("Using skin from config: {} (index: {})", name, idx);
                    return Self {
                        skins,
                        current_idx: idx,
                    };
                } else if !name.is_empty() {
                    info!("Skin '{}' from config not found, using default", name);
                }
            }
            0
        } else {
            0
        };
        Self { skins, current_idx }
    }

    pub fn get_current(&self) -> Option<&SkinEntry> {
        self.skins.get(self.current_idx)
    }

    pub fn next(&mut self) -> &SkinEntry {
        if !self.skins.is_empty() {
            self.current_idx = (self.current_idx + 1) % self.skins.len();
        }
        &self.skins[self.current_idx]
    }

    pub fn prev(&mut self) -> &SkinEntry {
        if !self.skins.is_empty() {
            self.current_idx = self.current_idx.saturating_sub(1);
        }
        &self.skins[self.current_idx]
    }

    pub fn get_index(&self) -> usize {
        self.current_idx
    }

    pub fn set_index(&mut self, idx: usize) {
        if idx < self.skins.len() {
            self.current_idx = idx;
        }
    }

    pub fn get_all_skins(&self) -> &[SkinEntry] {
        &self.skins
    }

    pub fn set_next_by_direction(&mut self, dir: Direction) -> usize {
        let delta = match dir {
            Direction::Right => 1isize,
            Direction::Left => -1isize,
        };
        self.current_idx =
            (self.current_idx as isize + delta).rem_euclid(self.skins.len() as isize) as usize;
        self.current_idx
    }
}
