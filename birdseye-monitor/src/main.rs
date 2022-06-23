mod client;
mod config;
mod platform;

use crate::client::process::monitor_processes;
use crate::platform::get_current_user;
use crate::config::load_config;
use sysinfo::SystemExt;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::fmt()
        .with_env_filter("debug")
        .init();

    // Load application configuration
    let _config = load_config();

    let mut stream = monitor_processes();

    info!("Current user is: {:?}", get_current_user());

    for usr in sysinfo::System::default().users() {
        info!("{:?}", usr);
    }

    while let Some(value) = stream.recv().await {
        info!("{:?}", value);
    }
}
