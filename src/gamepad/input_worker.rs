use gilrs::Gilrs;
use std::time::Duration;
use tokio::{task::JoinHandle, time::interval};
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tracing::info;

use super::listener::InputListener;
use crate::error::AppResult;

pub struct RawInputWorker<L: InputListener> {
    gilrs: Gilrs,
    listener: L,
}

impl<L: InputListener> RawInputWorker<L> {
    pub fn build(listener: L) -> AppResult<Self>
    where
        L: InputListener + 'static,
    {
        let gilrs = Gilrs::new()?;
        Ok(Self { gilrs, listener })
    }

    pub fn run(self, tracker: &TaskTracker, shutdown_token: CancellationToken) -> JoinHandle<()>
    where
        L: InputListener + 'static,
    {
        let mut gilrs = self.gilrs;
        let mut listener = self.listener;
        tracker.spawn(async move {
            let mut interval = interval(Duration::from_millis(16));
            while !shutdown_token.is_cancelled() {
                interval.tick().await;

                while let Some(raw_event) = gilrs.next_event() {
                    listener.handle_raw(raw_event);
                }
                listener.tick();
            }
            info!("Input worker shutting down");
        })
    }
}
