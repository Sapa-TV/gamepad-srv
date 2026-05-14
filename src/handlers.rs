use axum::{
    Json,
    extract::State,
    extract::ws::WebSocketUpgrade,
    response::{Html, IntoResponse, Response},
};
use tokio::fs;
use tracing::info;

use crate::websocket::handler::handle_socket;
use crate::{app::AppState, skin_manager::discovery::SkinEntry};

pub async fn index_handler() -> Html<String> {
    match fs::read_to_string("assets/index.html").await {
        Ok(contents) => Html(contents),
        Err(_) => Html("Cant find file error".into()),
    }
}

pub async fn skin_handler(State(state): State<AppState>) -> Response {
    if let Some(info) = state.skin_manager.lock().unwrap().get_current_info() {
        return axum::Json(info).into_response();
    }
    (
        axum::http::StatusCode::SERVICE_UNAVAILABLE,
        "Skin not loaded",
    )
        .into_response()
}

pub async fn list_skins_handler(State(state): State<AppState>) -> Json<Vec<SkinEntry>> {
    Json(state.skin_manager.lock().unwrap().get_all_skins().to_vec())
}

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    if state
        .shutting_down
        .load(std::sync::atomic::Ordering::SeqCst)
    {
        info!("Rejecting WebSocket connection: server shutting down");
        return (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            "Server shutting down",
        )
            .into_response();
    }

    let rx = state.channels.ws_sender().subscribe();
    let gamepad_state = state.gamepad_state.clone();
    ws.on_upgrade(move |socket| handle_socket(socket, gamepad_state, rx))
}
