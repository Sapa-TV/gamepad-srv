use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::broadcast;

use gilrs::Button;
use tracing::debug;

use crate::events::AppEvent;
use crate::gamepad_state::GamepadEvent;
use crate::skin::SkinEntry;
use crate::skin_change_state::Direction;

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
    skins: Vec<SkinEntry>,
    current_skin_index: Arc<Mutex<usize>>,
    ws_tx: Arc<broadcast::Sender<GamepadEvent>>,
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
            Ok(AppEvent::SkinChange(dir)) => {
                let new_idx = {
                    let mut idx = current_skin_index.lock().unwrap();
                    let delta = match dir {
                        Direction::Right => 1isize,
                        Direction::Left => -1isize,
                    };
                    *idx = (*idx as isize + delta).rem_euclid(skins.len() as isize) as usize;
                    *idx
                };

                if let Ok(info) = crate::skin::load_skin_info(&skins[new_idx].dir_name) {
                    debug!("Skin change: {} -> index: {}", info.name, new_idx);
                    let _ = ws_tx.send(GamepadEvent::SkinChanged {
                        name: info.name,
                        path: info.path,
                        index: new_idx,
                    });
                }
            }
            Err(_) => break,
        }
    }
}
