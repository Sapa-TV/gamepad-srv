#![feature(nonpoison_mutex)]
#![feature(sync_nonpoison)]
#![feature(bool_to_result)]

use crate::{app::manager::AppManager, error::AppResult};

mod app;
mod config;
mod error;
mod gamepad;
mod server;
mod skin_manager;

#[tokio::main(flavor = "current_thread")]
async fn main() -> AppResult<()> {
    enable_ansi_support::enable_ansi_support().ok();

    let mut debug_directive: Option<&str> = None;
    // debug_directive = Some("info,gamepad_srv::app::command_worker=debug");

    let default_level = if cfg!(debug_assertions) {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    let directive: String = match debug_directive {
        Some(dbg_dir) => dbg_dir.to_string(),
        None => default_level.to_string(),
    };

    let filter = tracing_subscriber::EnvFilter::new(directive);

    tracing_subscriber::fmt().with_env_filter(filter).init();

    println!("Gamepad server: dual pc gameviewer for OBS");

    AppManager::build().await?.run().await
}
