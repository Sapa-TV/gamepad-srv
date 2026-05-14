use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use tracing::debug;

use crate::events::AppEvent;
use crate::gamepad::state::GamepadEvent;
use crate::skin_manager::manager::SkinManager;

pub async fn run_button_actions(
    mut rx: broadcast::Receiver<AppEvent>,
    mut skin_manager: SkinManager,
    ws_tx: Arc<broadcast::Sender<GamepadEvent>>,
    save_tx: Arc<Mutex<Option<mpsc::Sender<String>>>>,
) {
    loop {
        match rx.recv().await {
            Ok(AppEvent::SkinChange(dir)) => {
                let new_idx = skin_manager.set_next_by_direction(dir);

                if let Some((skin, info)) = skin_manager.get_current_full() {
                    debug!("Skin change: {} -> index: {}", info.name, new_idx);
                    let _ = ws_tx.send(GamepadEvent::SkinChanged {
                        name: info.name,
                        path: info.path,
                        index: new_idx,
                    });

                    let tx_guard = save_tx.lock().unwrap();
                    if let Some(ref tx) = *tx_guard {
                        let _ = tx.try_send(skin.dir_name.clone());
                    }
                }
            }
            Err(_) => break,
            _ => {}
        }
    }
}
