use std::sync::Arc;

use serde::Serialize;
use tokio::{
    sync::{broadcast, mpsc},
    task::JoinHandle,
};
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tracing::{debug, error, info};

use crate::{
    app::mode_state::{AppModeEnum, AppModeStateMachine},
    error::AppResult,
    gamepad::event::GamepadEvent,
    server::WsInput,
    skin_manager::manager::SkinManager,
};

#[derive(Debug, Clone, Serialize)]
pub enum AppCommand {
    #[serde(rename = "sw")]
    SkinSwitch(String),
    #[serde(rename = "ssr")]
    SkinSwitchReady,
    #[serde(rename = "act")]
    SkinSwitchActivate,
    #[serde(rename = "dea")]
    SkinSwitchDeactivate,
    #[serde(rename = "sh")]
    AppShutdown,
}

impl TryFrom<AppModeEnum> for AppCommand {
    type Error = ();

    fn try_from(value: AppModeEnum) -> Result<Self, Self::Error> {
        use AppModeEnum::*;
        match value {
            SkinChangeActive => Ok(AppCommand::SkinSwitchActivate),
            Normal => Ok(AppCommand::SkinSwitchDeactivate),
            _ => Err(()),
        }
    }
}

pub struct CommandWorker {
    ctrl_rx: mpsc::Receiver<GamepadEvent>,
    ws_tx: broadcast::Sender<WsInput>,
    skin_manager: Arc<SkinManager>,
}

impl CommandWorker {
    pub fn build(
        skin_manager: Arc<SkinManager>,
        ctrl_rx: mpsc::Receiver<GamepadEvent>,
        ws_tx: broadcast::Sender<WsInput>,
    ) -> AppResult<Self> {
        Ok(Self {
            skin_manager,
            ctrl_rx,
            ws_tx,
        })
    }

    pub fn run(self, tracker: &TaskTracker, shutdown_token: CancellationToken) -> JoinHandle<()> {
        let mut input_rx = self.ctrl_rx;
        let (ack_tx, mut ack_rx) = mpsc::channel::<AppModeEnum>(4);
        let ws_tx = self.ws_tx;
        let shutdown_token_clone = shutdown_token.clone();
        tracker.spawn(async move {
            while let Some(event) = ack_rx.recv().await {
                if shutdown_token_clone.is_cancelled() {
                    break;
                }

                let Ok(cmd) = AppCommand::try_from(event) else {
                    continue;
                };

                if let Err(err) = ws_tx.send(cmd.into()) {
                    error!("Failed to send command: {err}");
                }
            }
            info!("Ack worker shutting down");
        });
        tracker.spawn(async move {
            let mut state = AppModeStateMachine::new(ack_tx);
            while let Some(event) = input_rx.recv().await {
                if shutdown_token.is_cancelled() {
                    break;
                }

                if let GamepadEvent::Ignored = event {
                    continue;
                }

                debug!("Received event: {:?}", event);
                state.update(&event);

                // self.ctrl_tx.try_send(event);
                // self.ws_tx.send(self.state.into());
            }
            info!("Command worker shutting down");
        })
    }
}
