use std::sync::Arc;
use tokio::sync::broadcast;

use gilrs::Button;

use crate::events::AppEvent;

pub trait ButtonHandler: Send + Sync {
    fn on_pressed(&self, button: Button);
    fn on_released(&self, button: Button);
}

pub struct ButtonAction {
    pub button: Button,
    pub handler: Arc<dyn ButtonHandler>,
}

pub async fn run_button_actions(
    mut rx: broadcast::Receiver<AppEvent>,
    actions: Vec<ButtonAction>,
) {
    loop {
        match rx.recv().await {
            Ok(AppEvent::Gilrs(event)) => {
                use gilrs::EventType;
                match event.event {
                    EventType::ButtonPressed(btn, _) => {
                        for action in &actions {
                            if action.button == btn {
                                action.handler.on_pressed(btn);
                            }
                        }
                    }
                    EventType::ButtonReleased(btn, _) => {
                        for action in &actions {
                            if action.button == btn {
                                action.handler.on_released(btn);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Err(_) => break,
        }
    }
}