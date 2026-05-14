use axum::extract::ws::WebSocket;
use serde_json::to_string;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::signal;
use tokio::sync::broadcast;
use tracing::info;

use crate::gamepad::state::{GamepadEvent, GamepadState};

pub async fn handle_socket(
    mut socket: WebSocket,
    state: Arc<Mutex<GamepadState>>,
    mut rx: broadcast::Receiver<GamepadEvent>,
) {
    info!("WebSocket client connected");

    let output = {
        let s = state.lock().unwrap();
        s.to_output()
    };
    match to_string(&output) {
        Ok(json) => {
            if socket.send(json.into()).await.is_err() {
                info!("WebSocket client disconnected");
            }
        }
        Err(e) => {
            tracing::error!("Failed to serialize: {}", e);
        }
    }

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                info!("Ctrl+C received, closing websocket");
                break;
            }
            event = rx.recv() => {
                match event {
                    Ok(e) => {
                        match to_string(&vec![e]) {
                            Ok(json) => {
                                if socket.send(json.into()).await.is_err() {
                                    info!("WebSocket client disconnected");
                                    break;
                                }
                            }
                            Err(e) => {
                                tracing::error!("Failed to serialize: {}", e);
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    }

    info!("Websocket closed");
}
