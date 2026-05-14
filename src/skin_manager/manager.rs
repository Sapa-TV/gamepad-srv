use super::discovery::{SkinEntry, SkinInfo, discover_skins, load_skin_info};
use crate::skin_switch::state::Direction;

pub struct SkinManager {
    skins: Vec<SkinEntry>,
    current_idx: usize,
}

impl SkinManager {
    pub fn discover_with_config(skin_from_config: Option<String>) -> Self {
        let skins = discover_skins();
        let current_idx = if !skins.is_empty() {
            if let Some(name) = &skin_from_config {
                if let Some(idx) = skins.iter().position(|s| &s.dir_name == name) {
                    return Self {
                        skins,
                        current_idx: idx,
                    };
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

    pub fn get_current_info(&self) -> Option<SkinInfo> {
        self.skins
            .get(self.current_idx)
            .and_then(|s| load_skin_info(&s.dir_name).ok())
    }

    pub fn get_current_full(&self) -> Option<(&SkinEntry, SkinInfo)> {
        self.skins
            .get(self.current_idx)
            .and_then(|s| load_skin_info(&s.dir_name).ok().map(|info| (s, info)))
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
