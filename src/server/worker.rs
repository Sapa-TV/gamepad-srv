use axum::{Router, routing::get};
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::sync::Arc;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;
use tower_http::services::ServeDir;
use tracing::info;

use crate::error::AppError;
use crate::error::AppResult;
use crate::server::ServerState;
use crate::server::handlers::{index_handler, list_skins_handler, skin_handler};
use crate::server::ws_upgrade::ws_upgrade_handler;

#[non_exhaustive]
pub struct ServerWorker {
    addr: SocketAddr,
    local_ip: IpAddr,
}

impl ServerWorker {
    pub fn build(port: u16) -> AppResult<Self> {
        let addr: SocketAddr = format!("0.0.0.0:{}", port)
            .to_socket_addrs()
            .map_err(|err| AppError::Server(format!("get address error: {err}")))?
            .next()
            .ok_or_else(|| AppError::Server("No addresses found".into()))?;

        let local_ip =
            local_ip_address::local_ip().unwrap_or_else(|_| "127.0.0.1".parse().unwrap());

        Ok(Self { addr, local_ip })
    }

    pub fn run(
        self,
        tracker: &TaskTracker,
        shutdown_token: CancellationToken,
        state: ServerState,
    ) -> JoinHandle<AppResult<()>> {
        let addr = self.addr;
        let local_ip = self.local_ip;
        tracker.spawn(async move {
            let listener = tokio::net::TcpListener::bind(addr)
                .await
                .map_err(|err| AppError::Server(format!("failed to bind to address {err}")))?;

            let skin_manager = { Arc::clone(&state.skin_manager) };

            let skin_router = Router::new()
                .route("/skin", get(skin_handler))
                .route("/list_skins", get(list_skins_handler))
                .with_state(skin_manager);

            let ws_router = Router::new()
                .route("/ws", get(ws_upgrade_handler))
                .with_state(state);

            let app = Router::new()
                .route("/", get(index_handler))
                .merge(skin_router)
                .merge(ws_router)
                .fallback_service(ServeDir::new("assets"));

            info!("Server starting on:");
            info!("http://localhost:{}", addr.port());
            info!("http://{}:{}", local_ip, addr.port());

            axum::serve(listener, app)
                .with_graceful_shutdown(async move {
                    shutdown_token.cancelled().await;
                    info!("Server shutting down");
                })
                .await
                .map_err(|err| AppError::Server(format!("run error {err}")))
        })
    }
}
