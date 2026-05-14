use std::net::ToSocketAddrs;
use std::sync::atomic::AtomicBool;
use std::{net::SocketAddr, sync::Arc, time::Duration};

use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;
use tracing::info;

use crate::app::{Channels, create_app_state};
use crate::handlers::{index_handler, list_skins_handler, skin_handler, ws_handler};
use tokio::signal;
use tokio::sync::mpsc;

mod app;
mod button_actions;
mod config;
mod events;
mod gamepad;
mod handlers;
mod skin_manager;
mod skin_switch;
mod tasks;
mod websocket;

#[tokio::main]
async fn main() {
    enable_ansi_support::enable_ansi_support().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::DEBUG.into()),
        )
        .init();

    let config = config::load_or_create_config().unwrap_or_else(|e| {
        tracing::warn!("Failed to load config: {}, using defaults", e);
        config::Config::default()
    });

    let addr: SocketAddr = format!("0.0.0.0:{}", config.port)
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let local_ip = local_ip_address::local_ip().unwrap_or_else(|_| "127.0.0.1".parse().unwrap());

    let channels = Channels::new();
    let app_state = create_app_state(channels, config.skin.clone());

    if let Some(skin) = app_state.skin_manager.lock().unwrap().get_current() {
        let mut cfg = config.clone();
        cfg.skin = Some(skin.dir_name.clone());
        let _ = config::save_config(&cfg);
    }

    let (save_tx, mut save_rx) = mpsc::channel::<String>(32);

    app_state.channels.spawn_all_tasks(
        app_state.gamepad_state.clone(),
        app_state.skin_manager.clone(),
        save_tx,
    );

    let shutting_down = app_state.shutting_down.clone();

    tokio::spawn(async move {
        while let Some(skin_name) = save_rx.recv().await {
            if let Ok(mut cfg) = config::load_or_create_config() {
                cfg.skin = Some(skin_name);
                if let Err(e) = config::save_config(&cfg) {
                    tracing::warn!("Failed to save config: {}", e);
                }
            }
        }
    });

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/ws", get(ws_handler))
        .route("/skin", get(skin_handler))
        .route("/list_skins", get(list_skins_handler))
        .with_state(app_state)
        .fallback_service(ServeDir::new("assets"));

    info!("Server starting on:");
    info!("  http://localhost:{}", addr.port());
    info!("  http://{}:{}", local_ip, addr.port());

    axum::serve(listener, app)
        .with_graceful_shutdown(graceful_shutdown(shutting_down))
        .await
        .unwrap();
}

async fn graceful_shutdown(shutting_down: Arc<AtomicBool>) {
    signal::ctrl_c().await.expect("Cant handle Ctrl+C");
    info!("Ctrl+C received, web server exiting...");
    shutting_down.store(true, std::sync::atomic::Ordering::SeqCst);
    tokio::time::sleep(Duration::from_secs(1)).await;
}
