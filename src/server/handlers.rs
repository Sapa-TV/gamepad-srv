use std::sync::Arc;

use axum::{
    extract::State,
    response::{Html, IntoResponse, Response},
};
use tokio::fs;

use crate::skins::SkinViewer;

// use crate::skin_manager::manager::SkinManager;

pub async fn index_handler() -> Html<String> {
    match fs::read_to_string("assets/index.html").await {
        Ok(contents) => Html(contents),
        Err(_) => Html("Cant find file error".into()),
    }
}

pub async fn skin_handler<SV>(State(skin_viewer): State<Arc<SV>>) -> Response
where
    SV: SkinViewer,
{
    let skin = skin_viewer.current_skin();
    if let Some(skin) = skin {
        return axum::Json(skin.clone()).into_response();
    }
    (
        axum::http::StatusCode::SERVICE_UNAVAILABLE,
        "Skin not loaded",
    )
        .into_response()
}
