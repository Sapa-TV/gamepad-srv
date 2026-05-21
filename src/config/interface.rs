use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
    pub skin_path: String,
}

pub trait ConfigInterface: Send + Sync + 'static {
    fn save_skin(&self, path: &str);
    fn current_skin(&self) -> String;
}
