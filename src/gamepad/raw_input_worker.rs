use std::time::Duration;

use gilrs::Gilrs;
use tokio::{sync::mpsc::Sender, task::JoinHandle, time::interval};
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tracing::{error, info};

use crate::{error::AppResult, gamepad::event::GamepadEvent};

pub struct RawInputWorker {
    gilrs: gilrs::Gilrs,
    raw_tx: Sender<GamepadEvent>,
}

impl RawInputWorker {
    pub fn build(raw_tx: Sender<GamepadEvent>) -> AppResult<Self> {
        let gilrs = Gilrs::new()?;
        Ok(Self { gilrs, raw_tx })
    }
    pub fn run(
        mut self,
        tracker: &TaskTracker,
        shutdown_token: CancellationToken,
    ) -> JoinHandle<()> {
        tracker.spawn(async move {
            let mut interval = interval(Duration::from_millis(16));
            while !shutdown_token.is_cancelled() {
                interval.tick().await;
                while let Some(raw_event) = self.gilrs.next_event() {
                    let app_event: GamepadEvent = raw_event.event.into();
                    if let Err(err) = self.raw_tx.try_send(app_event) {
                        error!("Internal error: {:?}", err);
                        break;
                    }
                }
            }
            info!("Input worker shutting down");
        })
    }
}
