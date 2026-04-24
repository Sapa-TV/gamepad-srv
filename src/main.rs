use std::{net::SocketAddr, time::Duration, sync::Arc};
use std::net::ToSocketAddrs;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;

use crate::gamepad_state::{GamepadEvent, GamepadState};
use crate::event_processor::process_event;
use axum::{
    Router,
    extract::{State as AxumState, WebSocketUpgrade, ws::WebSocket},
    response::{Html, Response, IntoResponse},
    routing::get,
};
use gilrs::Gilrs;
use serde_json::to_string;
use tokio::{fs, signal, time};
use tokio::sync::broadcast;
use tower_http::services::ServeDir;
use tracing::{info, debug, error};

mod gamepad_state;
mod event_processor;

#[derive(Clone)]
struct AppState {
    gamepad_state: Arc<Mutex<GamepadState>>,
    tx: Arc<broadcast::Sender<GamepadEvent>>,
    shutting_down: Arc<AtomicBool>,
}

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
    
    let gamepad_state: Arc<Mutex<GamepadState>> = Arc::new(Mutex::new(GamepadState::new()));
    let gamepad_state_clone = gamepad_state.clone();

    let (tx, _rx) = broadcast::channel(100);
    let tx = Arc::new(tx);
    let tx_clone = tx.clone();
    
    let tick_tx = tx.clone();
    let tick_state = gamepad_state.clone();

    tokio::spawn(async move {
        loop {
            time::sleep(Duration::from_millis(50)).await;
            let sticks = {
                let s = tick_state.lock().unwrap();
                GamepadEvent::Sticks {
                    lx: s.left_x,
                    ly: s.left_y,
                    rx: s.right_x,
                    ry: s.right_y,
                }
            };
            let _ = tick_tx.send(sticks);
        }
    });

    tokio::spawn(async move {
        let mut gilrs = match Gilrs::new() {
            Ok(g) => g,
            Err(e) => {
                error!("Failed to initialize gilrs: {}", e);
                return;
            }
        };

        info!("Gamepad polling started");

        loop {
            while let Some(event) = gilrs.next_event() {
                let mut state = gamepad_state_clone.lock().unwrap();
                if let Some(gamepad_event) = process_event(&mut state, event) {
                    debug!("Gamepad event: {:?}", gamepad_event);
                    let _ = tx_clone.send(gamepad_event);
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(16)).await;
        }
    });

    let shutting_down = Arc::new(AtomicBool::new(false));
    let shutting_down_clone = shutting_down.clone();

    let app_state = AppState {
        gamepad_state,
        tx,
        shutting_down,
    };

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/ws", get(ws_handler))
        .with_state(app_state)
        .fallback_service(ServeDir::new("assets"));

    info!("Server starting on:");
    info!("  http://localhost:{}", addr.port());
    info!("  http://{}:{}", local_ip, addr.port());

    axum::serve(listener, app)
        .with_graceful_shutdown(graceful_shutdown(shutting_down_clone))
        .await
        .unwrap();
}

async fn graceful_shutdown(shutting_down: Arc<AtomicBool>) {
    signal::ctrl_c().await.expect("Cant handle Ctrl+C");
    info!("Ctrl+C received, web server exiting...");
    shutting_down.store(true, std::sync::atomic::Ordering::SeqCst);
    tokio::time::sleep(Duration::from_secs(1)).await;
}

async fn index_handler() -> Html<String> {
    match fs::read_to_string("assets/index.html").await {
        Ok(contents) => Html(contents),
        Err(_) => Html("Cant find file error".into()),
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade, 
    AxumState(state): AxumState<AppState>
) -> Response {
    if state.shutting_down.load(std::sync::atomic::Ordering::SeqCst) {
        info!("Rejecting WebSocket connection: server shutting down");
        return (axum::http::StatusCode::SERVICE_UNAVAILABLE, "Server shutting down").into_response();
    }
    
    let rx = state.tx.subscribe();
    let gamepad_state = state.gamepad_state.clone();
    ws.on_upgrade(move |socket| handle_socket(socket, gamepad_state, rx))
}

async fn handle_socket(
    mut socket: WebSocket, 
    state: Arc<Mutex<GamepadState>>,
    mut rx: broadcast::Receiver<GamepadEvent>
) {
    info!("WebSocket client connected");

    let output = {
        let s = state.lock().unwrap();
        s.to_output()
    };
    let _ = socket.send(to_string(&output).unwrap().into()).await;

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                info!("Ctrl+C received, closing websocket");
                break;
            }
            event = rx.recv() => {
                match event {
                    Ok(e) => {
                        if socket.send(to_string(&vec![e]).unwrap().into()).await.is_err() {
                            info!("WebSocket client disconnected");
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    }

    info!("Websocket closed");
}