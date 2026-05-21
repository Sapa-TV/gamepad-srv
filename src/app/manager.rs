use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tracing::{error, info};

use crate::{
    error::AppResult,
    gamepad::{AppInputListener, AppInputMapper, RawInputWorker},
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

        // TODO: start app manager
        let input_mapper = AppInputMapper::new();
        let input_converter = AppInputListener::build(input_mapper);
        let input_worker = RawInputWorker::build(input_converter)?;

        // Run all workers
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
