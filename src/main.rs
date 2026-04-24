use std::{net::SocketAddr, time::Duration};
use std::net::ToSocketAddrs;

use crate::gamepad_state::{process_event, GamepadState};
use axum::{
    Router,
    extract::{WebSocketUpgrade, ws::WebSocket},
    response::{Html, Response},
    routing::get,
};
use gilrs::Gilrs;
use serde_json::to_string;
use tokio::{fs, signal};
use tower_http::services::ServeDir;
use tracing::{info, debug, error};

mod gamepad_state;

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
    
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/ws", get(ws_handler))
        .fallback_service(ServeDir::new("assets"));

    info!("Server starting on:");
    info!("  http://localhost:{}", addr.port());
    info!("  http://{}:{}", local_ip, addr.port());

    axum::serve(listener, app)
        .with_graceful_shutdown(graceful_shutdown())
        .await
        .unwrap();
}

async fn graceful_shutdown() {
    signal::ctrl_c().await.expect("Cant handle Ctrl+C");
    info!("Ctrl+C received, web server exiting...");
    tokio::time::sleep(Duration::from_secs(1)).await;
}

async fn index_handler() -> Html<String> {
    match fs::read_to_string("assets/index.html").await {
        Ok(contents) => Html(contents),
        Err(_) => Html("Cant find file error".into()),
    }
}

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    info!("WebSocket client connected");

    let mut gilrs = match Gilrs::new() {
        Ok(g) => g,
        Err(e) => {
            error!("Failed to initialize gilrs: {}", e);
            return;
        }
    };

    let mut state = GamepadState::new();
    let mut gamepad_id: Option<gilrs::GamepadId> = None;

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                info!("Ctrl+C received, closing websocket");
                break;
            }
            _ = async {
                while let Some(event) = gilrs.next_event() {
                    if gamepad_id.is_none() {
                        gamepad_id = Some(event.id);
                        info!("Gamepad connected: {:?}", event.id);
                    }

                    process_event(&mut state, event);
                    let output = state.to_output();
                    debug!("State: left=({}, {}) right=({}, {}) buttons={:?}", 
                        output.left_x, output.left_y, output.right_x, output.right_y, output.buttons);
                    let _ = socket.send(to_string(&output).unwrap().into()).await;
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(16)).await;
            } => {}
        }
    }

    info!("Websocket closed");
}