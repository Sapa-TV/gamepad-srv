use std::sync::Arc;

use tokio::sync::broadcast;
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tracing::{error, info};

use crate::{
    app::AppState,
    error::AppResult,
    gamepad::{
        gamepad_store::GamepadStore, input_worker::RawInputWorker, listener::AppInputListener,
        mapper::AppInputMapper,
    },
    server::{WsInput, worker::ServerWorker, ws_sender::AppWsSender},
    skins::skin_manager::AppSkinManager,
};

#[non_exhaustive]
pub struct AppManager {}

impl AppManager {
    pub async fn build() -> AppResult<Self> {
        info!("[App] Init requirements");
        // TODO: build app manager

        Ok(Self {})
    }

    pub async fn run(self) -> AppResult<()> {
        info!("[App] Starting");

        let shutdown_token = CancellationToken::new();
        let tracker = TaskTracker::new();

        let (ws_tx, ws_rx) = broadcast::channel::<WsInput>(20);

        // TODO: start app manager
        let ws_sender = AppWsSender::new(ws_tx.clone());

        let skin_manager = AppSkinManager::builder(ws_sender.clone()).build().await?;
        let skin_manager = Arc::new(skin_manager);
        let app = AppState::new(Arc::clone(&skin_manager), ws_sender.clone());
        let gamepad_state = GamepadStore::new(ws_sender);

        let input_mapper = AppInputMapper::new(app);
        let input_listener = AppInputListener::build(input_mapper, gamepad_state);
        let input_worker = RawInputWorker::build(input_listener)?;
        let server = ServerWorker::build(3000, skin_manager)?;

        // Run all workers
        server.run(&tracker, shutdown_token.clone());
        input_worker.run(&tracker, shutdown_token.clone());
        self.run_ctrl_c_worker(&tracker, shutdown_token);

        info!("[App] All tasks started");
        tracker.close();
        info!("[App] Wait all tasks");
        tracker.wait().await;

        info!("[App] Stopped");
        Ok(())
    }

    fn run_ctrl_c_worker(&self, tracker: &TaskTracker, shutdown_token: CancellationToken) {
        tracker.spawn(async move {
            if let Err(err) = tokio::signal::ctrl_c().await {
                error!("Ctrl+C signal error: {}", err);
            }
            info!("[App] Ctrl+C signal received, stopping");
            shutdown_token.cancel();
        });
    }
}
