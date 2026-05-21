use serde::Serialize;

pub trait SkinNavigator: Send + Sync + 'static {
    fn next_skin(&self);
    fn prev_skin(&self);
}

pub trait SkinViewer: Send + Sync + 'static {
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
