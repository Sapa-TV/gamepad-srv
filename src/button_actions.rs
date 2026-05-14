use std::sync::Arc;
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
    save_tx: mpsc::Sender<String>,
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

                    let _ = save_tx.send(skin.dir_name.clone()).await;
                }
            }
            Err(_) => break,
            _ => {}
        }
    }
}
