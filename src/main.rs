use std::net::ToSocketAddrs;
use std::sync::atomic::AtomicBool;
use std::{net::SocketAddr, sync::Arc, time::Duration};

use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;
use tracing::info;

use crate::app::{Channels, create_app_state};
use crate::handlers::{index_handler, list_skins_handler, skin_handler, ws_handler};
use crate::tasks::spawn_stick_tick;
use tokio::signal;

mod app;
mod button_actions;
mod event_processor;
mod events;
mod gamepad_state;
mod handlers;
mod skin;
mod tasks;
mod ws;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::DEBUG.into()),
        )
        .init();

    let addr: SocketAddr = "0.0.0.0:3000".to_socket_addrs().unwrap().next().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let local_ip = local_ip_address::local_ip().unwrap_or_else(|_| "127.0.0.1".parse().unwrap());

    let channels = Channels::new();
    let app_state = create_app_state(channels.ws_sender());

    let tick_state = app_state.gamepad_state.clone();
    let tick_ws_tx = channels.ws_sender();
    spawn_stick_tick(tick_state, tick_ws_tx);

    let gilrs_state = app_state.gamepad_state.clone();
    channels.spawn_all_tasks(gilrs_state);

    let shutting_down = app_state.shutting_down.clone();

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
