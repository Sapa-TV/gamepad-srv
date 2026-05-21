use serde::Serialize;

pub trait SkinNavigator: Send + Sync {
    fn next_skin(&mut self);
    fn prev_skin(&mut self);
}

pub trait SkinViewer: Send + Sync {
    fn current_skin(&self) -> Option<&Skin>;
}

#[non_exhaustive]
#[derive(Debug, Clone, Serialize)]
pub struct Skin {
    pub name: String,
    pub path: String,
}

impl Skin {
    pub fn new(name: String, path: String) -> Self {
        Self { name, path }
    }
}
