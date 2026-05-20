use tracing::debug;

pub trait InputListener: Send + Sync {
    fn handle_raw(&mut self, event: gilrs::Event) {
        debug!("Raw event: {:?}", event);
    }
}

#[non_exhaustive]
pub struct AppInputListener {}

impl InputListener for AppInputListener {
    fn handle_raw(&mut self, event: gilrs::Event) {
        // TODO: Implement default input converter
        debug!("Raw event: {:?}", event);
    }
}

impl AppInputListener {
    pub fn build() -> Self {
        Self {}
    }
}
