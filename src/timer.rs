use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct GameTimer(Instant);

impl GameTimer {
    pub fn new() -> Self {
        GameTimer(Instant::now())
    }

    pub fn elapsed(&self) -> Duration {
        self.0.elapsed()
    }
}
