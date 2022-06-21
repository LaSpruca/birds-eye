mod client;
mod config;

use crate::client::current_user::get_current_user;
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

    info!("Current user is: {:?}", get_current_user());

    for usr in sysinfo::System::default().users() {
        info!("{:?}", usr);
    }

    while let Some(value) = stream.recv().await {
        info!("{:?}", value);
    }
}
