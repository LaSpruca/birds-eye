mod client;
mod config;

use crate::client::process::monitor_processes;
use crate::config::load_config;
use sysinfo::SystemExt;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::fmt()
        .with_env_filter("debug")
        .init();

    // Load application configuration
    let config = load_config();

    let mut stream = monitor_processes();

    info!("Listing users");
    for usr in sysinfo::System::default().users() {
        info!("{:?}", usr);
    }

    while let Some(value) = stream.recv().await {
        info!("{:?}", value);
    }
}
