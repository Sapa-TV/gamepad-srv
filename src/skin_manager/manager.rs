use super::discovery::SkinEntry;

pub struct SkinManager {
    skins: Vec<SkinEntry>,
    current_idx: usize,
}

impl SkinManager {
    pub fn new(skins: Vec<SkinEntry>) -> Self {
        let current_idx = 0;
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
}
