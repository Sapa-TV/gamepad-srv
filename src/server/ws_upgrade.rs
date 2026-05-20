use axum::{extract::State, extract::ws::WebSocketUpgrade, response::Response};

use crate::server::{ServerState, ws_worker::ws_worker};

pub async fn ws_upgrade_handler(
    ws: WebSocketUpgrade,
    State(state): State<ServerState>,
) -> Response {
    // if state
    //     .shutting_down
    //     .load(std::sync::atomic::Ordering::SeqCst)
    // {
    //     info!("Rejecting WebSocket connection: server shutting down");
    //     return (
    //         axum::http::StatusCode::SERVICE_UNAVAILABLE,
    //         "Server shutting down",
    //     )
    //         .into_response();
    // }
    let shutdown_token = state.shutdown_token;
    let ws_rx = state.ws_tx.subscribe();

    ws.on_upgrade(move |socket| ws_worker(socket, shutdown_token, ws_rx))
}
