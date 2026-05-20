use tokio::{
    sync::{broadcast, mpsc},
    task::JoinHandle,
};
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tracing::{debug, error, info};

use crate::{
    error::AppResult,
    gamepad::{event::GamepadEvent, state::GamepadState},
    server::WsInput,
};

pub struct InputHubWorker {
    raw_rx: mpsc::Receiver<GamepadEvent>,
    ctrl_tx: mpsc::Sender<GamepadEvent>,
    ws_tx: broadcast::Sender<WsInput>,
}

impl InputHubWorker {
    pub fn build(
        raw_rx: mpsc::Receiver<GamepadEvent>,
        ctrl_tx: mpsc::Sender<GamepadEvent>,
        ws_tx: broadcast::Sender<WsInput>,
    ) -> AppResult<Self> {
        Ok(Self {
            raw_rx,
            ctrl_tx,
            ws_tx,
        })
    }
    pub fn run(self, tracker: &TaskTracker, shutdown_token: CancellationToken) -> JoinHandle<()> {
        let mut raw_rx = self.raw_rx;
        let mut state = GamepadState::new();
        tracker.spawn(async move {
            while let Some(event) = raw_rx.recv().await {
                if shutdown_token.is_cancelled() {
                    break;
                }

                if let GamepadEvent::Ignored = event {
                    continue;
                }

                debug!("Received event: {:?}", event);
                state.update(&event);

                if let Err(err) = self.ctrl_tx.try_send(event) {
                    error!("Internal channel error: {err}");
                };

                if let Err(_) = self.ws_tx.send(state.into()) {
                    debug!("No ws clients connected");
                }
            }
            info!("Input hub worker shutting down");
        })
    }
}
