use std::{fs, path::Path};
use tracing::{debug, error, warn};

use crate::{
    config::{Config, ConfigInterface},
    error::{AppError, AppResult},
};

const CONFIG_FILE: &str = "config.toml";

impl Config {
    pub fn load() -> Self {
        let loaded = Self::try_load();
        match loaded {
            Ok(config) => config,
            Err(_) => {
                let config = Self::default();
                if let Err(err) = config.save() {
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

    pub fn save(&self) -> AppResult<()> {
        let contents = toml::to_string_pretty(&self)
            .map_err(|err| AppError::Config(format!("Config parse error: {err}")))?;
        fs::write(CONFIG_FILE, contents)
            .map_err(|err| AppError::Config(format!("Config write error: {err}")))?;
        debug!("Saved config to {}", CONFIG_FILE);
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 3000,
            skin_path: String::new(),
        }
    }
}

impl ConfigInterface for Config {
    fn save_skin(&self, path: &str) {
        let mut config = Config::load();
        config.skin_path = path.to_string();
        if let Err(err) = config.save() {
            error!("Failed to save skin to config: {err}");
        }
    }

    fn current_skin(&self) -> String {
        self.skin_path.clone()
    }
}
