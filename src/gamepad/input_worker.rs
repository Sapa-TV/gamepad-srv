use gilrs::Gilrs;
use std::time::Duration;
use tokio::{task::JoinHandle, time::interval};
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tracing::info;

use super::listener::InputListener;
use crate::error::AppResult;

pub struct RawInputWorker<L: InputListener> {
    gilrs: Gilrs,
    input_converter: L,
}

impl<L: InputListener> RawInputWorker<L> {
    pub fn build(input_converter: L) -> AppResult<Self>
    where
        L: InputListener + 'static,
    {
        let gilrs = Gilrs::new()?;
        Ok(Self {
            gilrs,
            input_converter,
        })
    }

    pub fn run(self, tracker: &TaskTracker, shutdown_token: CancellationToken) -> JoinHandle<()>
    where
        L: InputListener + 'static,
    {
        let mut gilrs = self.gilrs;
        let mut input_converter = self.input_converter;
        tracker.spawn(async move {
            let mut interval = interval(Duration::from_millis(16));
            while !shutdown_token.is_cancelled() {
                interval.tick().await;
                while let Some(raw_event) = gilrs.next_event() {
                    input_converter.handle_raw(raw_event);
                }
            }
            info!("Input worker shutting down");
        })
    }
}
