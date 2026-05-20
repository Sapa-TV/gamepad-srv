use std::sync::{Arc, nonpoison::Mutex};
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use tracing::debug;

use crate::events::AppEvent;
use crate::gamepad::state::GamepadEvent;
use crate::skin_manager::manager::SkinManager;

pub async fn run_button_actions(
    mut rx: broadcast::Receiver<AppEvent>,
    skin_manager: Arc<Mutex<SkinManager>>,
    ws_tx: Arc<broadcast::Sender<GamepadEvent>>,
    save_tx: mpsc::Sender<String>,
) {
    loop {
        match rx.recv().await {
            Ok(AppEvent::SkinChange(dir)) => {
                let (skin, info) = {
                    let mut sm = skin_manager.lock();
                    sm.set_next_by_direction(dir);
                    let (skin, info) = match sm.get_current_full() {
                        Some(result) => result,
                        None => {
                            tracing::warn!("No current skin loaded");
                            return;
                        }
                    };
                    (skin.clone(), info)
                };

                debug!("Skin change: {}", info.name);
                let _ = ws_tx.send(GamepadEvent::SkinChanged {
                    name: info.name,
                    path: info.path,
                });

                let _ = save_tx.send(skin.dir_name.clone()).await;
            }
            Err(_) => break,
            _ => {}
        }
    }
}
