use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;
use tracing::{debug, info};

const CONFIG_FILE: &str = "config.toml";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub skin: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 3000,
            skin: None,
        }
    }
}

pub fn load_or_create_config() -> io::Result<Config> {
    let path = Path::new(CONFIG_FILE);

    if path.exists() {
        let contents = fs::read_to_string(path)?;
        match toml::from_str(&contents) {
            Ok(config) => {
                debug!("Loaded config from {}", CONFIG_FILE);
                Ok(config)
            }
            Err(e) => {
                info!("Failed to parse config.toml: {}, using defaults", e);
                Ok(Config::default())
            }
        }
    } else {
        let default_config = Config::default();
        save_config(&default_config)?;
        info!("Created default config.toml");
        Ok(default_config)
    }
}

pub fn save_config(config: &Config) -> io::Result<()> {
    let contents = toml::to_string_pretty(config)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    fs::write(CONFIG_FILE, contents)?;
    debug!("Saved config to {}", CONFIG_FILE);
    Ok(())
}
