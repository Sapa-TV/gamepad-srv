use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use tracing::{debug, warn};

use crate::error::{AppError, AppResult};

const CONFIG_FILE: &str = "config.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    port: u16,
    skin: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 3000,
            skin: None,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let loaded = Self::try_load();
        match loaded {
            Ok(config) => config,
            Err(_) => {
                let config = Self::default();
                let save_result = config.save();
                if let Err(err) = save_result {
                    warn!("Failed to save config: {err}");
                }
                config
            }
        }
    }

    fn try_load() -> AppResult<Self> {
        let path = Path::new(CONFIG_FILE);

        if !path.exists() {
            return Err(AppError::Config("Config file does not exist".into()));
        }

        let contents = fs::read_to_string(path)
            .map_err(|err| AppError::Config(format!("Config file read error: {err}")))?;

        match toml::from_str(&contents) {
            Ok(config) => {
                debug!("Loaded config from {}", CONFIG_FILE);
                Ok(config)
            }
            Err(e) => {
                warn!("Failed to parse config.toml: {}, using defaults", e);
                Ok(Config::default())
            }
        }
    }

    fn save(&self) -> AppResult<()> {
        let contents = toml::to_string_pretty(&self)
            .map_err(|err| AppError::Config(format!("Config parse error: {err}")))?;
        fs::write(CONFIG_FILE, contents)
            .map_err(|err| AppError::Config(format!("Config write error: {err}")))?;
        debug!("Saved config to {}", CONFIG_FILE);
        Ok(())
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_skin_name(&self) -> &Option<String> {
        &self.skin
    }

    pub fn set_skin_name(&mut self, name: String) {
        // TODO: save to config
        self.skin = Some(name);
    }
}
