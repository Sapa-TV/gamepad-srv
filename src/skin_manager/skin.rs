use serde::Serialize;
use std::path::Path;
use tracing::debug;

use crate::{error::AppResult, skin_manager::validate::validate_get_name};

#[non_exhaustive]
#[derive(Debug, Clone, Serialize)]
pub struct Skin {
    name: String,
    path: String,
}

impl Skin {
    pub fn try_from_dir(path: &Path) -> AppResult<Self> {
        let path = path.to_owned();

        match validate_get_name(&path) {
            Ok(name) => {
                let path = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                debug!("Skin loaded ok, name: {}, path: {}", name, path);
                Ok(Self {
                    name,
                    path: format!("/skins/{}", path),
                })
            }
            Err(err) => {
                debug!("Skin invalid, path: {}", path.display());
                Err(err)
            }
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &String {
        &self.path
    }
}
