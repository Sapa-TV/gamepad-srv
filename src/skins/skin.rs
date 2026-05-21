use std::path::Path;
use tracing::debug;

use super::validate::validate_get_name;
use crate::{error::AppResult, skins::Skin};

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
                Ok(Self::new(name, format!("/skins/{}", path)))
            }
            Err(err) => {
                debug!("Skin invalid, path: {}", path.display());
                Err(err)
            }
        }
    }
}
