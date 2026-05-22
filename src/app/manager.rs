use std::sync::Arc;

use tokio::sync::broadcast;
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tracing::{error, info};

use crate::{
    app::AppState,
    config::Config,
    error::AppResult,
    gamepad::{GamepadStore, InputListener, InputMapper, RawInputWorker},
    server::{ServerState, ServerWorker, WsInput, WsSender},
    skins::SkinManager,
};

#[non_exhaustive]
pub struct AppManager {
    config: Config,
}

impl AppManager {
    pub async fn build() -> AppResult<Self> {
        info!("[App] Init requirements");

        Ok(Self {
            config: Config::load(),
        })
    }

    pub async fn run(self) -> AppResult<()> {
        info!("[App] Starting");

        let shutdown_token = CancellationToken::new();
        let tracker = TaskTracker::new();

        let (ws_tx, _) = broadcast::channel::<WsInput>(20);

        let ws_sender = WsSender::new(ws_tx.clone());
        let port = self.config.port;

        let skin_manager = SkinManager::builder(ws_sender.clone(), self.config)
            .build()
            .await?;
        let skin_manager = Arc::new(skin_manager);
        let app = AppState::new(Arc::clone(&skin_manager), ws_sender.clone());
        let gamepad_store = GamepadStore::new(ws_sender);

        let input_mapper = InputMapper::new(app);
        let input_listener = InputListener::build(input_mapper, gamepad_store);
        let input_worker = RawInputWorker::build(input_listener)?;
        let server = ServerWorker::build(port, skin_manager)?;

        let server_state = ServerState::new(ws_tx, shutdown_token.clone());

        // Run all workers
        server.run(&tracker, shutdown_token.clone(), server_state);
        input_worker.run(&tracker, shutdown_token.clone());
        Self::run_ctrl_c_worker(&tracker, shutdown_token);

        info!("[App] All tasks started");
        tracker.close();
        info!("[App] Wait all tasks");
        tracker.wait().await;

        info!("[App] Stopped");
        Ok(())
    }

    fn run_ctrl_c_worker(tracker: &TaskTracker, shutdown_token: CancellationToken) {
        tracker.spawn(async move {
            if let Err(err) = tokio::signal::ctrl_c().await {
                error!("Ctrl+C signal error: {}", err);
            }
            info!("[App] Ctrl+C signal received, stopping");
            shutdown_token.cancel();
        });
    }
}
