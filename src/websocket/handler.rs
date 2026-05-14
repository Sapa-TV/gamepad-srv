use std::sync::Arc;
use std::sync::Mutex;

use axum::extract::ws::WebSocket;
use serde_json::to_string;
use tokio::signal;
use tokio::sync::broadcast;
use tracing::info;

use crate::gamepad::state::GamepadEvent;

pub async fn handle_socket(
    mut socket: WebSocket,
    state: Arc<Mutex<crate::gamepad::state::GamepadState>>,
    mut rx: broadcast::Receiver<GamepadEvent>,
) {
    info!("WebSocket client connected");

    let output = {
        let s = state.lock().unwrap();
        s.to_output()
    };
    let _ = socket.send(to_string(&output).unwrap().into()).await;

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                info!("Ctrl+C received, closing websocket");
                break;
            }
            event = rx.recv() => {
                match event {
                    Ok(e) => {
                        if socket.send(to_string(&vec![e]).unwrap().into()).await.is_err() {
                            info!("WebSocket client disconnected");
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    }

    info!("Websocket closed");
}
