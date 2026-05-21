use axum::response::Html;
use tokio::fs;

// use crate::skin_manager::manager::SkinManager;

pub async fn index_handler() -> Html<String> {
    match fs::read_to_string("assets/index.html").await {
        Ok(contents) => Html(contents),
        Err(_) => Html("Cant find file error".into()),
    }
}
