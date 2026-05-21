use axum::{extract::State, extract::ws::WebSocketUpgrade, response::Response};

use crate::server::{ServerState, ws_worker::ws_worker};

pub async fn ws_upgrade_handler(
    ws: WebSocketUpgrade,
    State(state): State<ServerState>,
) -> Response {
    let shutdown_token = state.shutdown_token;
    let broadcast_rx = state.ws_tx.subscribe();

    ws.on_upgrade(move |socket| ws_worker(socket, shutdown_token, broadcast_rx))
}
