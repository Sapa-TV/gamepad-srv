use axum::{
    extract::State,
    response::{Html, IntoResponse, Response},
};
use std::sync::Arc;
use tokio::fs;

use crate::skin_manager::manager::SkinManager;

pub async fn index_handler() -> Html<String> {
    match fs::read_to_string("assets/index.html").await {
        Ok(contents) => Html(contents),
        Err(_) => Html("Cant find file error".into()),
    }
}

pub async fn skin_handler(State(skin_manager): State<Arc<SkinManager>>) -> Response {
    let skin = &skin_manager.get_current_skin();
    if let Some(skin) = skin {
        return axum::Json(skin).into_response();
    }
    (
        axum::http::StatusCode::SERVICE_UNAVAILABLE,
        "Skin not loaded",
    )
        .into_response()
}

pub async fn list_skins_handler(State(_skin_manager): State<Arc<SkinManager>>) -> Response {
    todo!("Need ui with skin names and visual selection")
}
