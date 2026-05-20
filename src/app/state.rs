use std::sync::Arc;
use tokio_util::sync::CancellationToken;

use crate::{config::Config, skin_manager::manager::SkinManager};

#[non_exhaustive]
#[derive(Debug)]
pub struct AppState {
    config: Config,
    shutdown_token: CancellationToken,
    pub skin_manager: Arc<SkinManager>,
}

impl AppState {
    pub fn builder() -> AppStateBuilder {
        AppStateBuilder::new()
    }

    pub fn get_config_port(&self) -> u16 {
        self.config.get_port()
    }

    pub fn get_shutdown_token(&self) -> CancellationToken {
        self.shutdown_token.clone()
    }
}

#[non_exhaustive]
#[derive(Default, Debug)]
pub struct AppStateBuilder {
    config: Option<Config>,
    skin_manager: Option<SkinManager>,
}

impl AppStateBuilder {
    fn new() -> Self {
        Self::default()
    }

    pub fn with_config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    pub fn with_skin_manager(mut self, skin_manager: SkinManager) -> Self {
        self.skin_manager = Some(skin_manager);
        self
    }

    pub fn build(self) -> AppState {
        let config = self.config.expect("Config is requierd");
        let skin_manager = self.skin_manager.expect("Skin manager is required");
        let skin_manager = Arc::new(skin_manager);
        let shutdown_token = CancellationToken::new();

        AppState {
            config,
            shutdown_token,
            skin_manager,
        }
    }
}
