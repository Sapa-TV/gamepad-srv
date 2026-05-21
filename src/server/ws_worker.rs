use axum::extract::ws::{Message, WebSocket};
use better_tokio_select::tokio_select;
use futures::{SinkExt, StreamExt};
use tokio::sync::broadcast::Receiver;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

use crate::server::WsInput;

pub async fn ws_worker(
    socket: WebSocket,
    shutdown_token: CancellationToken,
    mut broadcast_rx: Receiver<WsInput>,
) {
    info!("WebSocket client connected");

    let (mut ws_tx, mut ws_rx) = socket.split();

    loop {
        tokio_select!(match .. {
            .. if let msg = broadcast_rx.recv() => {
                match msg {
                    Ok(msg) => {
                        // debug!("Received input: {:?}", msg);

                        if let Err(err) = ws_tx.send(Message::Text(msg.into())).await {
                            error!("Error sending message: {:?}", err);
                            break;
                        }
                    }
                    Err(_) => {
                        error!("Broadcast channel for ws_worker closed");
                        break;
                    }
                }
            }
            .. if let msg = ws_rx.next() => {
                match msg {
                    Some(Ok(_)) => {
                        debug!("WS receiving message ignored");
                    }
                    Some(Err(e)) => {
                        error!("Error receiving message: {:?}", e);
                    }
                    None => {
                        break;
                    }
                }
            }
            .. if let _ = shutdown_token.cancelled() => {
                info!("Websocket handler shutting down");
                break;
            }
        });
    }

    info!("Websocket closed");
}
