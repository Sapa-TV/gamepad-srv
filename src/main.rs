use std::{net::SocketAddr, time::Duration};

use crate::gamepad_state::convert_state;
use axum::{
    Router,
    extract::{WebSocketUpgrade, ws::WebSocket},
    response::{Html, Response},
    routing::get,
};
use gamepad::GamepadEngine;
use serde_json::to_string;
use tokio::{fs, signal};
use tower_http::services::ServeDir;

mod gamepad_state;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/ws", get(ws_handler))
        .fallback_service(ServeDir::new("assets"));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(graceful_shutdown())
        .await
        .unwrap();
}

async fn graceful_shutdown() {
    signal::ctrl_c().await.expect("Cant handle Ctrl+C");
    println!("\r\nCtrl+C received, web server exiting...");
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
    println!("WebSocket client connected..");

    let mut gamepad_engine = GamepadEngine::new();

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                println!("\r\nCtrl+C received, closing websocket...");
                break;
            }
            _ = async {
                gamepad_engine.update().unwrap();

                let gamepad = gamepad_engine.gamepads().get(0);

                match gamepad {
                    Some(gamepad) => {
                       let state = convert_state(gamepad);
                    //    print!("\rGamepadstate: {:?}                                                ", state);
                       let _ = socket.send(to_string(&state).unwrap().into()).await;
                    }
                    None => {}
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(16)).await;
            } => {}
        }
    }

    println!("Websocket closed");
}
