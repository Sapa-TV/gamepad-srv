use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use tracing::debug;

use crate::events::AppEvent;
use crate::gamepad::state::GamepadEvent;
use crate::skin_manager::discovery::{SkinEntry, load_skin_info};
use crate::skin_switch::state::Direction;

pub async fn run_button_actions(
    mut rx: broadcast::Receiver<AppEvent>,
    skins: Vec<SkinEntry>,
    current_skin_index: Arc<Mutex<usize>>,
    ws_tx: Arc<broadcast::Sender<GamepadEvent>>,
    save_tx: Arc<Mutex<Option<mpsc::Sender<String>>>>,
) {
    loop {
        match rx.recv().await {
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

                if let Ok(info) = load_skin_info(&skins[new_idx].dir_name) {
                    debug!("Skin change: {} -> index: {}", info.name, new_idx);
                    let _ = ws_tx.send(GamepadEvent::SkinChanged {
                        name: info.name,
                        path: info.path,
                        index: new_idx,
                    });

                    let tx_guard = save_tx.lock().unwrap();
                    if let Some(ref tx) = *tx_guard {
                        let _ = tx.try_send(skins[new_idx].dir_name.clone());
                    }
                }
            }
            Err(_) => break,
            _ => {}
        }
    }
}
