use std::sync::{Arc, nonpoison::Mutex};
use tokio::sync::{broadcast, mpsc};
use tokio_util::task::TaskTracker;
use tracing::{error, info};

use crate::{
    app::{CommandWorker, state::AppState},
    config::Config,
    error::AppResult,
    gamepad::{
        event::GamepadEvent, input_hub_worker::InputHubWorker, raw_input_worker::RawInputWorker,
    },
    server::{ServerState, WsInput, worker::ServerWorker},
    skin_manager::manager::SkinManager,
};

pub struct AppManager {
    app_state: Arc<Mutex<AppState>>,
}

impl AppManager {
    pub async fn build() -> AppResult<Self> {
        info!("[App] Init requirements");

        let config = Config::new();
        let skin_manager = SkinManager::builder().build().await?;

        let app_state = AppState::builder()
            .with_config(config)
            .with_skin_manager(skin_manager)
            .build();

        let app_state = Arc::new(Mutex::new(app_state));

        Ok(Self { app_state })
    }

    pub async fn run(self) -> AppResult<()> {
        info!("[App] Starting");

        let tracker = TaskTracker::new();
        let shutdown_token = { self.app_state.lock().get_shutdown_token() };

        // TODO: spawn all othe tasks
        let (raw_to_hub_tx, raw_to_hub_rx) = mpsc::channel::<GamepadEvent>(20);
        let (hub_to_ctrl_tx, hub_to_ctrl_rx) = mpsc::channel::<GamepadEvent>(20);
        let (ws_tx, _) = broadcast::channel::<WsInput>(20);

        let skin_manager = { Arc::clone(&self.app_state.lock().skin_manager) };
        RawInputWorker::build(raw_to_hub_tx)?.run(&tracker, shutdown_token.clone());
        InputHubWorker::build(raw_to_hub_rx, hub_to_ctrl_tx, ws_tx.clone())?
            .run(&tracker, shutdown_token.clone());
        CommandWorker::build(skin_manager, hub_to_ctrl_rx, ws_tx.clone())?
            .run(&tracker, shutdown_token.clone());

        let port = { self.app_state.lock().get_config_port() };
        let skin_manager = { Arc::clone(&self.app_state.lock().skin_manager) };
        let server_state = ServerState::new(skin_manager, ws_tx, shutdown_token.clone());
        ServerWorker::build(port)?.run(&tracker, shutdown_token.clone(), server_state);

        tracker.spawn(async move {
            if let Err(err) = tokio::signal::ctrl_c().await {
                error!("Ctrl+C signal error: {}", err);
            }
            info!("[App] Ctrl+C signal received, stopping");
            shutdown_token.cancel();
        });

        info!("[App] All tasks started");
        tracker.close();
        info!("[App] Wait all tasks");
        tracker.wait().await;
        info!("[App] stopped");
        Ok(())
    }
}
