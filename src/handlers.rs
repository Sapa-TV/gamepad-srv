use axum::{
    Json,
    extract::State,
    extract::ws::WebSocketUpgrade,
    response::{Html, IntoResponse, Response},
};

use crate::app::AppState;
use crate::websocket::handler::handle_socket;
use tokio::fs;
use tracing::info;

pub async fn index_handler() -> Html<String> {
    match fs::read_to_string("assets/index.html").await {
        Ok(contents) => Html(contents),
        Err(_) => Html("Cant find file error".into()),
    }
}

pub async fn skin_handler(State(state): State<AppState>) -> Response {
    let idx = *state.current_skin_index.lock().unwrap();
    if idx < state.skins.len() {
        if let Ok(info) = crate::skin::load_skin_info(&state.skins[idx].dir_name) {
            return axum::Json(info).into_response();
        }
    }
    (
        axum::http::StatusCode::SERVICE_UNAVAILABLE,
        "Skin not loaded",
    )
        .into_response()
}

pub async fn list_skins_handler(
    State(state): State<AppState>,
) -> Json<Vec<crate::skin::SkinEntry>> {
    Json(state.skins.clone())
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

    let rx = state.ws_tx.subscribe();
    let gamepad_state = state.gamepad_state.clone();
    ws.on_upgrade(move |socket| handle_socket(socket, gamepad_state, rx))
}
