use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use tracing::debug;

use crate::events::AppEvent;
use crate::gamepad::state::GamepadEvent;

pub async fn run_button_actions(
    mut rx: broadcast::Receiver<AppEvent>,
    skin_manager: Arc<Mutex<crate::skin_manager::manager::SkinManager>>,
    ws_tx: Arc<broadcast::Sender<GamepadEvent>>,
    save_tx: mpsc::Sender<String>,
) {
    loop {
        match rx.recv().await {
            Ok(AppEvent::SkinChange(dir)) => {
                let (new_idx, skin, info) = {
                    let mut sm = skin_manager.lock().unwrap();
                    let new_idx = sm.set_next_by_direction(dir);
                    let (skin, info) = sm.get_current_full().unwrap();
                    (new_idx, skin.clone(), info)
                };

                debug!("Skin change: {} -> index: {}", info.name, new_idx);
                let _ = ws_tx.send(GamepadEvent::SkinChanged {
                    name: info.name,
                    path: info.path,
                    index: new_idx,
                });

                let _ = save_tx.send(skin.dir_name.clone()).await;
            }
            Err(_) => break,
            _ => {}
        }
    }
}
